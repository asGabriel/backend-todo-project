use uuid::Uuid;

use crate::domains::{
    error::{Error, Result},
    task::{CreateTaskDTO, Task, UpdateTaskDTO},
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

    pub async fn create_task(&self, task: CreateTaskDTO) -> Result<Task> {
        self.task_repository.create_tasks(task).await
    }

    pub async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Task> {
        self.task_repository
            .remove_task_by_id(task_id)
            .await?
            .ok_or(Error::TaskNotFound(task_id))
    }

    pub async fn update_task_by_id(&self, task_id: Uuid, payload: UpdateTaskDTO) -> Result<Task> {
        let mut result = self.get_task_by_id(task_id).await?;

        if let Some(list_id) = payload.task_list_id {
            self.task_list_repository.get_list_by_id(list_id).await?;
        }

        result.update_changes(payload);

        self.task_repository
            .update_task_by_id(result)
            .await?
            .ok_or(Error::TaskNotFound(task_id))
    }
}
