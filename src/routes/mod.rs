use axum::{http::StatusCode, response::IntoResponse, Router};
mod task;

use crate::{domains::error::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(task::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
