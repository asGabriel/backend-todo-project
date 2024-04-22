use crate::domains::{
    error::Result,
    task_list::{CreateTaskList, TaskList},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TaskListRepository {
    async fn create_task_list(&self, task_list: CreateTaskList) -> Result<TaskList>;
    async fn list_task_lists(&self) -> Result<Vec<TaskList>>;
}

#[async_trait::async_trait]
impl TaskListRepository for SqlxRepository {
    async fn create_task_list(&self, task_list: CreateTaskList) -> Result<TaskList> {
        let task_list = sqlx::query_as!(
            TaskList,
            r#"
            INSERT INTO TASKLISTS (
                TASK_LIST_ID, TITLE
            ) VALUES (
                $1, $2
            ) RETURNING *
            "#,
            Uuid::new_v4(),
            task_list.title
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task_list)
    }

    async fn list_task_lists(&self) -> Result<Vec<TaskList>> {
        let task_lists = sqlx::query_as!(
            TaskList,
            r#"
            SELECT * FROM TASKLISTS WHERE DELETED_AT IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(task_lists)
    }
}
