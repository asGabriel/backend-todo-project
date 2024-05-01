use std::sync::Arc;

mod domains;
mod handlers;
mod repositories;
mod routes;

use axum::Router;
use handlers::Handler;
use repositories::SqlxRepository;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Could not fetch database url");
    let port = std::env::var("PORT").expect("Could not fetch PORT data");
    let url = format!("0.0.0.0:{}", port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to database");

    let sqlx_repository = Arc::new(SqlxRepository::new(pool));
    let handler = Handler::new(sqlx_repository.clone(), sqlx_repository);

    let app: Router = routes::configure_routes().with_state(handler);

    let tcp_listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
