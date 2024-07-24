use service::sea_orm::DatabaseConnection;

pub struct TrumplyAt65Database {
    pub connection: DatabaseConnection,
}

impl TrumplyAt65Database {
    pub async fn new() -> Self {
        let conn = service::sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap())
            .await
            .expect("can not connect to database, how old are you!");

        TrumplyAt65Database { connection: conn }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
