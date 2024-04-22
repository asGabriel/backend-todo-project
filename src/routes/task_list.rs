use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{
    domains::{error::Result, task_list::CreateTaskList},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/list-task", Router::new().route("/", post(create_task_list)))
}

async fn create_task_list(
    State(handler): State<Handler>,
    Json(list): Json<CreateTaskList>,
) -> Result<impl IntoResponse> {
    let task_list = handler.create_task_list(list).await?;

    Ok(Json::from(task_list))
}
