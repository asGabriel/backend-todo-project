use uuid::Uuid;

use crate::domains::{
    error::{Error, Result},
    task::{CreateTask, Task},
};

use super::Handler;

impl Handler {
    pub async fn get_task_by_id(&self, task_id: Uuid) -> Result<Task> {
        self.task_repository
            .get_task_by_id(task_id)
            .await?
            .ok_or(Error::TaskNotFound(task_id))
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        self.task_repository.list_tasks().await
    }

    pub async fn create_task(&self, task: CreateTask) -> Result<Task> {
        self.task_repository.create_tasks(task).await
    }

    pub async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Task> {
        self.task_repository
            .remove_task_by_id(task_id)
            .await?
            .ok_or(Error::TaskNotFound(task_id))
    }
}
