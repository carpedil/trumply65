use std::{collections::HashMap, env, ops::Mul};

use common::{
    excel_helper::XlsxHelper,
    input::ExportSpecInput,
    output::{ColumnData, DbTableStruct, TableColumnsInfo, TableRawData},
    utils::get_user_tab_columns_sql,
};
use entity::connection_config::{self, Entity as ConnectionConfig};
use oracle::Connection;
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, Order, QueryFilter, QueryOrder};

pub struct TrumplyCcQueryService;

impl TrumplyCcQueryService {
    pub async fn get_all_tcc(db: &DbConn) -> Result<Vec<connection_config::Model>, DbErr> {
        ConnectionConfig::find()
            .order_by(connection_config::Column::CreatedAt, Order::Desc)
            .all(db)
            .await
    }

    pub async fn get_actived_tcc(db: &DbConn) -> Option<connection_config::Model> {
        ConnectionConfig::find()
            .filter(connection_config::Column::IsActive.eq(true))
            .one(db)
            .await
            .unwrap()
    }
}

pub struct DatasourceQueryService {
    conn: Connection,
}

#[allow(dead_code)]
impl DatasourceQueryService {
    fn new(connection_config: connection_config::Model) -> Self {
        let conn = Connection::connect(
            connection_config.username,
            connection_config.password,
            connection_config.url,
        )
        .expect("can not connect to datasource db check your network setting");
        Self { conn }
    }

    fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub async fn load_datasource_tables(db: &DbConn) -> Result<Vec<TableColumnsInfo>, DbErr> {
        let mut table_columns: HashMap<String, Vec<ColumnData>> = HashMap::new();
        if let Some(connection_config) = TrumplyCcQueryService::get_actived_tcc(db).await {
            println!(
                "active_tcc: {}, banned_table_list: {:#?}",
                connection_config.id, connection_config.abandoned_table_list
            );
            let banned_table_list = &connection_config.abandoned_table_list;
            let dq = DatasourceQueryService::new(connection_config.clone());
            let row = dq
                .conn
                .query(&get_user_tab_columns_sql(banned_table_list.clone()), &[]);
            let mut row_list: Vec<DbTableStruct> = vec![];
            for row in row.unwrap().into_iter() {
                let row = row.unwrap();
                let table_name: String = row.get("TABLE_NAME").unwrap();
                let column_name: String = row.get("COLUMN_NAME").unwrap();
                let data_type: String = row.get("DATA_TYPE").unwrap();
                let data_len: i32 = row.get("DATA_LENGTH").unwrap();
                let row_data = DbTableStruct {
                    table_name,
                    column_name,
                    data_type,
                    data_len,
                };
                row_list.push(row_data);
            }
            for row in row_list {
                table_columns
                    .entry(row.table_name)
                    .or_insert(Vec::new())
                    .push(ColumnData {
                        column_name: row.column_name,
                        data_type: row.data_type,
                        data_len: row.data_len,
                    });
            }

            let mut ptable_list: Vec<TableColumnsInfo> = table_columns
                .into_iter()
                .map(|(table_name, column_infos)| TableColumnsInfo {
                    table_name,
                    column_infos,
                })
                .collect();
            ptable_list.sort_by(|a, b| a.table_name.cmp(&b.table_name));
            return Ok(ptable_list);
        }
        Err(sea_orm::DbErr::Custom(
            "no actived connection_config to use".into(),
        ))
    }

    pub async fn dump_datasource_tables(
        db: &DbConn,
        dump_spec: Vec<ExportSpecInput>,
    ) -> Result<String, DbErr> {
        if let Some(connection_config) = TrumplyCcQueryService::get_actived_tcc(db).await {
            println!("active_tcc: {}", connection_config.id);
            let dq = DatasourceQueryService::new(connection_config);
            let xls_helper = XlsxHelper::new();

            for dump_spec in dump_spec.into_iter() {
                println!("table : {}", dump_spec.table_name);
                let mut worksheet = xls_helper
                    .wb
                    .add_worksheet(Some(&dump_spec.table_name.as_str()))
                    .unwrap();
                // write columns
                for (col_num, column) in dump_spec.headers.into_iter().enumerate() {
                    let _ = worksheet.write_string(
                        0,
                        col_num as u16,
                        &column.column_name,
                        Some(&XlsxHelper::headers_format()),
                    );
                }

                let rows = dq.conn.query(&dump_spec.query_sql, &[]).expect(
                    format!(
                        "Error fetching rows,table_name:{},sql:{}",
                        &dump_spec.table_name, &dump_spec.query_sql
                    )
                    .as_str(),
                );

                let column_info: Vec<(String, &oracle::sql_type::OracleType)> = rows
                    .column_info()
                    .iter()
                    .map(|ci| (ci.name().to_string(), ci.oracle_type()))
                    .collect();

                let rows = dq
                    .conn
                    .query(&dump_spec.query_sql, &[])
                    .expect("Error fetching rows");
                // 计算每列的最大宽度
                let mut col_widths = vec![0; column_info.len()];
                for row in rows {
                    match row {
                        Ok(r) => {
                            for (col_num, (col, _col_type)) in column_info.iter().enumerate() {
                                if let Some(value) =
                                    r.get::<&str, Option<String>>(&col).unwrap_or(None)
                                {
                                    let v1 = &value.chars().count();
                                    let v2 = col_widths[col_num];
                                    if !col_widths.contains(v1) || !col_widths.contains(&v2) {
                                        if v1 > &v2 {
                                            // println!("col_num = {}|v2 = {}|v1 = {}|col_name = {}|value = {}", col_num,v2, v1, col,value);
                                            col_widths[col_num] = *v1;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching row: {:?}", e);
                            break;
                        }
                    }
                }
                dbg!(&col_widths.clone());
                let rows = dq
                    .conn
                    .query(&dump_spec.query_sql, &[])
                    .expect("Error fetching rows");

                for (row_num, row) in rows.enumerate() {
                    match row {
                        Ok(r) => {
                            for (col_num, (col, col_type)) in column_info.iter().enumerate() {
                                if let Some(value) =
                                    r.get::<&str, Option<String>>(&col).unwrap_or(None)
                                {
                                    // println!("{}", value);
                                    match col_type {
                                        oracle::sql_type::OracleType::Number(_, _) => {
                                            worksheet
                                                .write_number(
                                                    (row_num + 1) as u32,
                                                    col_num as u16,
                                                    value.parse::<f64>().unwrap(),
                                                    Some(&XlsxHelper::format()),
                                                )
                                                .unwrap();
                                        }
                                        _ => {
                                            worksheet
                                                .write_string(
                                                    (row_num + 1) as u32,
                                                    col_num as u16,
                                                    &value.to_string(),
                                                    Some(&XlsxHelper::format()),
                                                )
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching row: {:?}", e);
                            break;
                        }
                    }
                }
                for (colum_num, widths) in col_widths.clone().into_iter().enumerate() {
                    println!("第{}列 ,widths = {} ", colum_num, widths);
                    worksheet
                        .set_column(
                            colum_num as u16,
                            colum_num as u16,
                            (widths as f64).mul(1.4),
                            Some(&XlsxHelper::format()),
                        )
                        .unwrap();
                }
                // 冻结首行
                worksheet.freeze_panes(1, 0);
            }
            // 关闭资源句柄
            xls_helper.wb.close().unwrap();
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let file_path = current_dir.join(xls_helper.file_name);
            return Ok(file_path.to_str().unwrap().to_string());
        }
        Err(sea_orm::DbErr::Custom(
            "no actived connection_config to use".into(),
        ))
    }

    pub async fn get_table_data(
        db: &DbConn,
        input: ExportSpecInput,
    ) -> Result<Vec<TableRawData>, DbErr> {
        if let Some(connection_config) = TrumplyCcQueryService::get_actived_tcc(db).await {
            let dqs = DatasourceQueryService::new(connection_config);
            let rows = dqs
                .conn
                .query(&input.query_sql, &[])
                .map_err(|e| DbErr::Custom(e.to_string()))?;

            let _pt = TableColumnsInfo::new(&input.table_name, vec![]);
            let mut data_list: Vec<TableRawData> = vec![];
            for (_, row) in rows.enumerate() {
                match row {
                    Ok(r) => {
                        // let mut data: BTreeMap<String, String> = BTreeMap::new();
                        let mut row_value_list: Vec<String> = vec![];
                        for (_, col) in input.headers.iter().enumerate() {
                            if let Some(value) = r
                                .get::<&str, Option<String>>(&col.column_name)
                                .unwrap_or_default()
                            {
                                // data.insert(col.to_string(), value);
                                row_value_list.push(value)
                            } else {
                                row_value_list.push(String::new())
                            }
                        }
                        data_list.push(TableRawData::new(row_value_list))
                    }
                    Err(e) => return Err(sea_orm::DbErr::Custom(e.to_string())),
                }
            }
            return Ok(data_list);
        }
        Err(sea_orm::DbErr::Custom(
            "no actived connection_config to use".into(),
        ))
    }
}
