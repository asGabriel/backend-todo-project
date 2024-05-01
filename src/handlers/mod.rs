use std::sync::Arc;
pub mod task;
pub mod task_list;

use crate::repositories::{task::TaskRepository, task_list::TaskListRepository};

#[derive(Clone)]
pub struct Handler {
    task_repository: Arc<dyn TaskRepository + Send + Sync>,
    task_list_repository: Arc<dyn TaskListRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        task_repository: Arc<dyn TaskRepository + Send + Sync>,
        list_repository: Arc<dyn TaskListRepository + Send + Sync>,
    ) -> Self {
        Self {
            task_repository,
            task_list_repository: list_repository,
        }
    }
}
