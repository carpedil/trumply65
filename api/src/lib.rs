use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    self,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use dotenvy::dotenv;
use entity::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use graphql::schema::{build_schema, AppSchema};

pub mod error_handing;
pub mod graphql;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let schema = build_schema().await;
    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema);
    let srv_host = std::env::var("HOST").unwrap_or("localhost".to_string());
    let srv_port = std::env::var("PORT").unwrap_or("18000".to_string());
    let srv_url = format!("{}:{}", srv_host, srv_port);
    println!("Playground: http://{}/api/graphql", srv_url);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(srv_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}
