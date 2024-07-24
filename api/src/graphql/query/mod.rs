pub mod db_query;
pub mod tcc_query;

pub use db_query::DatasourceDbQuery;
use entity::async_graphql;
pub use tcc_query::TrumplyTccQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(TrumplyTccQuery, DatasourceDbQuery);
