use crate::Todo;
use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone, FromRef)]
pub struct Repository {
    db_pool: PgPool,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as!(Todo, r#"SELECT id, title, completed FROM todos"#,)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(todos)
    }

    pub async fn create_todo(&self, title: String, completed: bool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING id, title, completed"#,
            title,
            completed,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(())
    }
}
