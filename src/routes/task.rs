use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{
        error::Result,
        task::{CreateTaskDTO, UpdateTaskDTO},
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/:task_id", get(get_task_by_id))
            .route("/", get(list_tasks))
            .route("/", post(create_task))
            .route("/:task_id", delete(remove_task_by_id))
            .route("/:task_id", patch(update_task_by_id)),
    )
}

async fn get_task_by_id(
    State(handler): State<Handler>,
    Path(task_id): Path<Uuid>,
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
    Json(task): Json<CreateTaskDTO>,
) -> Result<impl IntoResponse> {
    let task = handler.create_task(task).await?;

    Ok(Json::from(task))
}

async fn remove_task_by_id(
    State(handler): State<Handler>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let task = handler.remove_task_by_id(task_id).await?;

    Ok(Json::from(task))
}

async fn update_task_by_id(
    State(handler): State<Handler>,
    Path(task_id): Path<Uuid>,
    Json(task): Json<UpdateTaskDTO>,
) -> Result<impl IntoResponse> {
    let task = handler.update_task_by_id(task_id, task).await?;

    Ok(Json::from(task))
}
