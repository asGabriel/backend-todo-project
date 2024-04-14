use crate::domains::{
    error::Result,
    task::{CreateTask, Task},
};

use super::Handler;

impl Handler {
    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        self.task_repository.list_tasks().await
    }

    pub async fn create_task(&self, task: CreateTask) -> Result<Task> {
        self.task_repository.create_tasks(task).await
    }
}
