use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub task_id: Uuid,
    pub task_list_id: Uuid,
    pub title: String,
    pub status: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDTO {
    pub title: String,
    pub task_list_id: Uuid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskDTO {
    pub title: Option<String>,
    pub task_list_id: Option<Uuid>,
    pub status: Option<bool>,
}

impl Task {
    pub fn update_changes(&mut self, payload: UpdateTaskDTO) -> Self {
        Task {
            task_id: self.task_id,
            task_list_id: payload.task_list_id.unwrap_or(self.task_list_id),
            title: payload.title.unwrap_or(self.title.to_owned()),
            status: payload.status.unwrap_or(self.status),
            created_at: self.created_at,
            deleted_at: Some(self.deleted_at.unwrap_or_default()),
            updated_at: Some(self.updated_at.unwrap_or_default())
        }
    }
}
