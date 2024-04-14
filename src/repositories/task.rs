use super::SqlxRepository;

#[async_trait::async_trait]
pub trait TaskRepository {}

impl TaskRepository for SqlxRepository {}
