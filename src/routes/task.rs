use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    domains::{error::Result, task::CreateTask},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/", get(list_tasks))
            .route("/", post(create_task)),
    )
}

async fn list_tasks(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let tasks = handler.list_tasks().await?;

    Ok(Json::from(tasks))
}

async fn create_task(
    State(handler): State<Handler>,
    Json(task): Json<CreateTask>,
) -> Result<impl IntoResponse> {
    let task = handler.create_task(task).await?;

    Ok(Json::from(task))
}
