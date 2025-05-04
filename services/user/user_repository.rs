use axum::extract::FromRef;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::model::User;

#[derive(Clone, FromRef)]
pub struct UserRepository {
    pub db: PgPool,
}

impl UserRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT * FROM users",)
            .fetch_all(&self.db)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        let user = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let user = query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, email: String, password: String) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
            email,
            password
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
}
