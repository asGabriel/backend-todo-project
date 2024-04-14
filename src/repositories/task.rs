use crate::domains::{
    error::Result,
    task::{CreateTask, Task},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn list_tasks(&self) -> Result<Vec<Task>>;
    async fn create_tasks(&self, task: CreateTask) -> Result<Task>;
}

#[async_trait::async_trait]
impl TaskRepository for SqlxRepository {
    async fn list_tasks(&self) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT * FROM TASKS
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
}
