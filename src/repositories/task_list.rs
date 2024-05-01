use crate::domains::{
    error::Result,
    task_list::{CreateTaskListDTO, TaskList, UpdateTaskListDTO},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TaskListRepository {
    async fn create_task_list(&self, task_list: CreateTaskListDTO) -> Result<TaskList>;
    async fn list_task_lists(&self) -> Result<Vec<TaskList>>;
    async fn get_list_by_id(&self, task_list_id: Uuid) -> Result<Option<TaskList>>;
    async fn update_task_list_by_id(
        &self,
        task_list: UpdateTaskListDTO,
        task_list_id: Uuid,
    ) -> Result<Option<TaskList>>;
    async fn delete_task_list_by_id(&self, task_list_id: Uuid) -> Result<Option<TaskList>>;
}

#[async_trait::async_trait]
impl TaskListRepository for SqlxRepository {
    async fn create_task_list(&self, task_list: CreateTaskListDTO) -> Result<TaskList> {
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

    async fn get_list_by_id(&self, task_list_id: Uuid) -> Result<Option<TaskList>> {
        let task_list = sqlx::query_as!(
            TaskList,
            r#"
            SELECT * FROM TASKLISTS WHERE DELETED_AT IS NULL AND TASK_LIST_ID=$1
            "#,
            task_list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task_list)
    }

    async fn update_task_list_by_id(
        &self,
        task_list: UpdateTaskListDTO,
        task_list_id: Uuid,
    ) -> Result<Option<TaskList>> {
        let task_list = sqlx::query_as!(
            TaskList,
            r#"
            UPDATE TASKLISTS SET
                TITLE=$1,
                UPDATED_AT=now()
            WHERE
                DELETED_AT IS NULL AND
                TASK_LIST_ID=$2
            RETURNING *
            "#,
            task_list.title,
            task_list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task_list)
    }

    async fn delete_task_list_by_id(&self, task_list_id: Uuid) -> Result<Option<TaskList>> {
        let task_list = sqlx::query_as!(
            TaskList,
            r#"
            UPDATE TASKLISTS SET
                DELETED_AT=now(),
                UPDATED_AT=now()
            WHERE
                DELETED_AT IS NULL AND
                TASK_LIST_ID=$1
            RETURNING *
            "#,
            task_list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task_list)
    }
}
