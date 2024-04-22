use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Databse error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Task not found")]
    TaskNotFound(Uuid),
    #[error("Task List not found")]
    TaskListNotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
