use axum::{extract::State, routing::get, Router};

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().route("/", get(create_task))
}

async fn create_task(State(handler): State<Handler>) {
    println!("this method will create a new task")
}
