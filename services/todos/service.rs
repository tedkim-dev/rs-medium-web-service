mod repository;
pub use repository::*;

mod model;
pub use model::*;

use axum::extract::FromRef;
mod tests;

/**
 * Service is a struct that contains a repository.
 * It is used to abstract the repository from the controller.
 * It is also used to add business logic to the repository.
 */
#[derive(Clone, FromRef)]
pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        self.repository
            .get_todos()
            .await
            .map_err(Error::GetTodosFailed)
    }

    pub async fn create_todo(&self, title: String) -> Result<(), Error> {
        self.repository
            .create_todo(title, false)
            .await
            .map_err(Error::CreateTodoFailed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get todos")]
    GetTodosFailed(sqlx::Error),

    #[error("Failed to create todo")]
    CreateTodoFailed(sqlx::Error),
}
