use crate::domains::{
    error::Result,
    task_list::{CreateTaskList, TaskList},
};

use super::Handler;

impl Handler {
    pub async fn create_task_list(&self, task_list: CreateTaskList) -> Result<TaskList> {
        self.task_list_repository.create_task_list(task_list).await
    }
}