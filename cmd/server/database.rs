use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_db_pool() -> PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@0.0.0.0:5432/postgres".to_string());

    PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to DB")
}