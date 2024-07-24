use crate::error_handing::AppErr;
use common::{
    input::{ExportSpecInput, FilePath},
    output::{TableColumnsInfo, TableRawData},
};
use db::TrumplyAt65Database;
use entity::async_graphql::{Context, Object};
use service::DatasourceQueryService;

use entity::async_graphql;

#[derive(Default)]
pub struct DatasourceDbQuery;

#[Object]
impl DatasourceDbQuery {
    async fn load_datasource_tables(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<TableColumnsInfo>, AppErr> {
        let db = ctx
            .data::<TrumplyAt65Database>()
            .map_err(|e| AppErr::DatabaseErr(e.message.to_owned()))?;

        let data = DatasourceQueryService::load_datasource_tables(&db.get_connection())
            .await
            .map_err(|e| AppErr::Other(e.to_string()))?;

        Ok(data)
    }

    async fn dump_datasource_tables(
        &self,
        ctx: &Context<'_>,
        dump_spec: Vec<ExportSpecInput>,
    ) -> Result<FilePath, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        let data = DatasourceQueryService::dump_datasource_tables(db.get_connection(), dump_spec)
            .await
            .map_err(|e| AppErr::Other(e.to_string()))?;
        Ok(data)
    }

    async fn get_table_data(
        &self,
        ctx: &Context<'_>,
        table_info: ExportSpecInput,
    ) -> Result<Vec<TableRawData>, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();

        let data = DatasourceQueryService::get_table_data(db.get_connection(), table_info)
            .await
            .map_err(|e| AppErr::NotFound(e.to_string()))?;

        Ok(data)
    }
}
