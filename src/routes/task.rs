use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{error::Result, task::CreateTask},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/:task_id", get(get_task_by_id))
            .route("/", get(list_tasks))
            .route("/", post(create_task)),
    )
}

async fn get_task_by_id(
    State(handler): State<Handler>,
    Path(task_id): Path<Uuid>
) -> Result<impl IntoResponse> {
    let task = handler.get_task_by_id(task_id).await?;

    Ok(Json::from(task))
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
