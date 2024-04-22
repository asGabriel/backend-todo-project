use axum::{http::StatusCode, response::IntoResponse, Router};
mod task;
mod task_list;

use crate::{domains::error::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new()
        .merge(task::configure_routes())
        .merge(task_list::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::TaskNotFound(task_id) => (
                StatusCode::NOT_FOUND,
                format!("Task id {task_id:?} not found"),
            ),
        }
        .into_response()
    }
}
