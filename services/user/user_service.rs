use axum::extract::FromRef;
use sqlx::PgPool;
use user_repository::UserRepository;
use thiserror::Error;

mod model;
pub use model::*;

mod user_repository;
mod get_users;
mod get_user_by_id;

#[derive(Clone, FromRef)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        let repo = UserRepository::new(db);
        Self {
            repo,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get users")]
    GetUsersFailed(sqlx::Error),
    #[error("Failed to get user by id")]
    GetUserByIdFailed(sqlx::Error),
}
