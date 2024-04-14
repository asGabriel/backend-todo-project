use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{domains::error::Result, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/tasks", Router::new().route("/", get(asd)))
}

async fn asd(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let task = handler.list_tasks().await?;

    Ok(Json::from(task))
}
