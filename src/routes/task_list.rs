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
        task_list::{CreateTaskListDTO, UpdateTaskListDTO},
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/task-lists",
        Router::new()
            .route("/", post(create_task_list))
            .route("/", get(list_task_lists))
            .route("/:task_list_id", get(get_list_by_id))
            .route("/:task_list_id", patch(update_task_list_by_id))
            .route("/:task_list_id", delete(delete_task_list_by_id)),
    )
}

async fn create_task_list(
    State(handler): State<Handler>,
    Json(list): Json<CreateTaskListDTO>,
) -> Result<impl IntoResponse> {
    let task_list = handler.create_task_list(list).await?;

    Ok(Json::from(task_list))
}

async fn list_task_lists(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let task_lists = handler.list_task_lists().await?;

    Ok(Json::from(task_lists))
}

async fn get_list_by_id(
    State(handler): State<Handler>,
    Path(task_list_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let task_list = handler.get_list_by_id(task_list_id).await?;

    Ok(Json::from(task_list))
}

async fn update_task_list_by_id(
    State(handler): State<Handler>,
    Path(task_list_id): Path<Uuid>,
    Json(task_list): Json<UpdateTaskListDTO>,
) -> Result<impl IntoResponse> {
    let task_list = handler
        .update_task_list_by_id(task_list, task_list_id)
        .await?;

    Ok(Json::from(task_list))
}

async fn delete_task_list_by_id(
    State(handler): State<Handler>,
    Path(task_list_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let task_list = handler.delete_task_list_by_id(task_list_id).await?;

    Ok(Json::from(task_list))
}
