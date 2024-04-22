use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    domains::{error::Result, task_list::CreateTaskList},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/task-lists",
        Router::new()
            .route("/", post(create_task_list))
            .route("/", get(list_task_lists)),
    )
}

async fn create_task_list(
    State(handler): State<Handler>,
    Json(list): Json<CreateTaskList>,
) -> Result<impl IntoResponse> {
    let task_list = handler.create_task_list(list).await?;

    Ok(Json::from(task_list))
}

async fn list_task_lists(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let task_lists = handler.list_task_lists().await?;

    Ok(Json::from(task_lists))
}
