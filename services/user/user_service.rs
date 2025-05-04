use axum::extract::FromRef;
use sqlx::PgPool;
use thiserror::Error;
use user_repository::UserRepository;

mod model;
pub use model::*;

mod get_user_by_id;
mod get_users;
mod user_repository;

#[derive(Clone, FromRef)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        let repo = UserRepository::new(db);
        Self { repo }
    }
}

impl UserService {
    pub async fn get_user_by_email(&self, email: String) -> Result<User, Error> {
        let user = self
            .repo
            .get_user_by_email(email)
            .await
            .map_err(Error::GetUserByEmailFailed)?;
        Ok(user)
    }

    pub async fn create_user(&self, email: String, password: String) -> Result<User, Error> {
        let user = self
            .repo
            .create_user(email, password)
            .await
            .map_err(Error::CreateUserFailed)?;
        Ok(user)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get users")]
    GetUsersFailed(sqlx::Error),
    #[error("Failed to get user by id")]
    GetUserByIdFailed(sqlx::Error),
    #[error("Failed to get user by email")]
    GetUserByEmailFailed(sqlx::Error),
    #[error("Failed to create user")]
    CreateUserFailed(sqlx::Error),
}
