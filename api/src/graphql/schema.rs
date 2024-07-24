use db::TrumplyAt65Database;
use entity::async_graphql::{EmptySubscription, Schema};
use migration::{Migrator, MigratorTrait};

use super::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = TrumplyAt65Database::new().await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
