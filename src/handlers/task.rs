use crate::domains::{error::Result, task::Task};

use super::Handler;

impl Handler {
    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        self.task_repository.list_tasks().await
    }
}
