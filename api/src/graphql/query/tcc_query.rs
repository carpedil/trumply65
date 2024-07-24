use async_graphql::{Context, Object};
use db::TrumplyAt65Database;
use entity::{async_graphql, connection_config};
use service::TrumplyCcQueryService;

use crate::error_handing::AppErr;

#[derive(Default)]
pub struct TrumplyTccQuery;

#[Object]
impl TrumplyTccQuery {
    async fn get_all_tcc(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<connection_config::Model>, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        Ok(TrumplyCcQueryService::get_all_tcc(db.get_connection())
            .await
            .map_err(|e| AppErr::Other(e.to_string()))?)
    }
}
