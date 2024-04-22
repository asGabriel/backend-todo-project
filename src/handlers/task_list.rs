use uuid::Uuid;

use crate::domains::{
    error::{Error, Result},
    task_list::{CreateTaskListDTO, TaskList, UpdateTaskListDTO},
};

use super::Handler;

impl Handler {
    pub async fn create_task_list(&self, task_list: CreateTaskListDTO) -> Result<TaskList> {
        self.task_list_repository.create_task_list(task_list).await
    }

    pub async fn list_task_lists(&self) -> Result<Vec<TaskList>> {
        self.task_list_repository.list_task_lists().await
    }

    pub async fn get_list_by_id(&self, task_list_id: Uuid) -> Result<TaskList> {
        self.task_list_repository
            .get_list_by_id(task_list_id)
            .await?
            .ok_or(Error::TaskListNotFound(task_list_id))
    }

    pub async fn update_task_list_by_id(
        &self,
        task_list: UpdateTaskListDTO,
        task_list_id: Uuid,
    ) -> Result<TaskList> {
        // self.get_list_by_id(task_list_id).await?;

        self.task_list_repository
            .update_task_list_by_id(task_list, task_list_id)
            .await?
            .ok_or(Error::TaskListNotFound(task_list_id))
    }

    pub async fn delete_task_list_by_id(&self, task_list_id: Uuid) -> Result<TaskList> {
        self.get_list_by_id(task_list_id).await?;

        self.task_list_repository
            .delete_task_list_by_id(task_list_id)
            .await?
            .ok_or(Error::TaskListNotFound(task_list_id))
    }
}
