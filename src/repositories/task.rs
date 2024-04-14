use crate::domains::{error::Result, task::Task};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn list_tasks(&self) -> Result<Vec<Task>>;
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
}
