/// USER_TAB_COLUMNS_SQL
pub fn get_user_tab_columns_sql(wc: Option<String>) -> String {
    if let Some(where_clause) = wc {
        return format!("SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE, DATA_LENGTH, NULLABLE FROM USER_TAB_COLUMNS WHERE TABLE_NAME NOT IN ({}) ORDER BY TABLE_NAME ,COLUMN_ID",where_clause);
    } else {
        return format!("SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE, DATA_LENGTH, NULLABLE FROM USER_TAB_COLUMNS ORDER BY TABLE_NAME ,COLUMN_ID");
    }
}

pub fn remove_target_with_commas(mut source: Vec<&str>, target: &str) -> String {
    source.retain(|item| *item != target);
    source
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
