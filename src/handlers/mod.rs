use std::sync::Arc;

use crate::repositories::task::TaskRepository;

#[derive(Clone)]
pub struct Handler {
    task_repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(task_repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }
}
