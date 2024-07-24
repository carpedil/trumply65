use super::async_graphql::{Context, Object};
use crate::error_handing::AppErr;
use common::input::{AbandedTableNameInput, CreateTccInput, MutationResult};
use db::TrumplyAt65Database;
use entity::{
    async_graphql::{self},
    connection_config,
};
use service::TrumplyCcMutationService;

#[derive(Default)]
pub struct RecipeMutation;

#[Object]
impl RecipeMutation {
    pub async fn create_tcc(
        &self,
        ctx: &Context<'_>,
        input: CreateTccInput,
    ) -> Result<connection_config::Model, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        println!("{:#?}", input);
        Ok(TrumplyCcMutationService::create_tcc(
            db.get_connection(),
            input.into_model_with_arbitrary_id(),
        )
        .await
        .map_err(|e| AppErr::DatabaseErr(e.to_string()))?)
    }

    pub async fn delete_tcc_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<MutationResult, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        println!("{:#?}", id);

        let res = TrumplyCcMutationService::delete_tcc_by_id(db.get_connection(), id)
            .await
            .map_err(|e| AppErr::DatabaseErr(e.to_string()))?;
        Ok(MutationResult {
            rows_affected: res.rows_affected,
        })
    }

    pub async fn active_tcc_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<connection_config::Model, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        println!("{:#?}", id);
        let res = TrumplyCcMutationService::active_tcc_by_id(db.get_connection(), id)
            .await
            .map_err(|e| AppErr::DatabaseErr(e.to_string()))?;
        Ok(res)
    }

    pub async fn update_aband_table_list(
        &self,
        ctx: &Context<'_>,
        banned_table: AbandedTableNameInput,
    ) -> Result<connection_config::Model, AppErr> {
        let db = ctx.data::<TrumplyAt65Database>().unwrap();
        println!("{:#?}", banned_table);
        let res =
            TrumplyCcMutationService::update_aband_table_list(db.get_connection(), banned_table)
                .await
                .map_err(|e| AppErr::DatabaseErr(e.to_string()))?;
        Ok(res)
    }

    pub async fn reset_banned_table_list(
        &self,
        ctx: &Context<'_>,
    ) -> Result<connection_config::Model, AppErr> {
        let db: &TrumplyAt65Database = ctx.data::<TrumplyAt65Database>().unwrap();
        let res: connection_config::Model =
            TrumplyCcMutationService::reset_banned_table_list(db.get_connection())
                .await
                .map_err(|e| AppErr::Other(e.to_string()))?;
        Ok(res)
    }
}
