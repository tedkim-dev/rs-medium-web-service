use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_db_pool() -> PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@0.0.0.0:5432/postgres".to_string());

    PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to DB")
}

// pub async fn connect(database_url: &str) -> Result<PgPool, Error> {
//     PgPoolOptions::new()
//         .max_connections(100)
//         .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
//         .connect(database_url)
//         .await
//         .map_err(|err| Error::ConnectingToDatabase(err.to_string()))
// }

// pub async fn migrate(db: &PgPool) -> Result<(), Error> {
//     match sqlx::migrate!("./migrations").run(db).await {
//         Ok(_) => Ok(()),
//         Err(err) => Err(err),
//     }?;

//     Ok(())
// }

// #[derive(thiserror::Error, Debug, Clone)]
// pub enum Error {
//     #[error("Bad config: {0}")]
//     BadConfig(String),
//     #[error("Connecting to database: {0}")]
//     ConnectingToDatabase(String),
//     #[error("Internal error: {0}")]
//     Internal(String),
//     #[error("Not found: {0}")]
//     NotFound(String),
//     #[error("Migrating database: {0}")]
//     DatabaseMigration(String),
// }

// impl std::convert::From<sqlx::Error> for Error {
//     fn from(err: sqlx::Error) -> Self {
//         match err {
//             sqlx::Error::RowNotFound => Error::NotFound("row not found".into()),
//             _ => Error::Internal(err.to_string()),
//         }
//     }
// }

// impl std::convert::From<sqlx::migrate::MigrateError> for Error {
//     fn from(err: sqlx::migrate::MigrateError) -> Self {
//         Error::DatabaseMigration(err.to_string())
//     }
// }