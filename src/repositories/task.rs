use crate::domains::{
    error::Result,
    task::{CreateTask, Task},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn get_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>>;
    async fn list_tasks(&self) -> Result<Vec<Task>>;
    async fn create_tasks(&self, task: CreateTask) -> Result<Task>;
    async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>>;
}

#[async_trait::async_trait]
impl TaskRepository for SqlxRepository {
    async fn get_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT * FROM TASKS WHERE DELETED_AT IS NULL AND TASK_ID=$1
            "#,
            task_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn list_tasks(&self) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT * FROM TASKS WHERE DELETED_AT IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn create_tasks(&self, task: CreateTask) -> Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO TASKS(TASK_ID, TITLE) VALUES($1, $2) RETURNING *
            "#,
            Uuid::new_v4(),
            task.title
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>> {
        let task = sqlx::query_as!(
            Task,
            r#"
            UPDATE TASKS SET DELETED_AT=NOW() WHERE TASK_ID=$1
            RETURNING *
            "#,
            task_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }
}
