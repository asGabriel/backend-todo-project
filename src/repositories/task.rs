use crate::domains::{
    error::Result,
    task::{CreateTaskDTO, Task},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TaskRepository {
    async fn get_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>>;
    async fn list_tasks(&self) -> Result<Vec<Task>>;
    async fn create_tasks(&self, task: CreateTaskDTO) -> Result<Task>;
    async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>>;
    async fn update_task_by_id(&self, task: Task) -> Result<Option<Task>>;
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

    async fn create_tasks(&self, task: CreateTaskDTO) -> Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO TASKS(TASK_ID, TITLE, TASK_LIST_ID) VALUES($1, $2, $3) RETURNING *
            "#,
            Uuid::new_v4(),
            task.title,
            task.task_list_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn remove_task_by_id(&self, task_id: Uuid) -> Result<Option<Task>> {
        let task = sqlx::query_as!(
            Task,
            r#"
            UPDATE TASKS SET DELETED_AT=NOW() WHERE DELETED_AT IS NULL AND TASK_ID=$1
            RETURNING *
            "#,
            task_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn update_task_by_id(&self, task: Task) -> Result<Option<Task>> {
        let task = sqlx::query_as!(
            Task,
            r#"
            UPDATE TASKS SET TITLE=$1, TASK_LIST_ID=$2, STATUS=$3, UPDATED_AT=NOW() WHERE DELETED_AT IS NULL AND TASK_ID=$4
            RETURNING *
            "#,
            task.title,
            task.task_list_id,
            task.status,
            task.task_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }
}
