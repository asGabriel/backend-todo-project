use axum::Router;
mod task;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(task::configure_routes())
}
