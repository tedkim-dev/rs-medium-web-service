use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use todo_service::{Repository as TodosRepository, Service as TodosService};

use crate::webapp::new_api_router;

#[derive(Clone, FromRef)]
pub struct ServerState {
    pub db_pool: PgPool,
    pub todos_service: TodosService,
}

impl ServerState {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool: db_pool.clone(),
            todos_service: TodosService::new(TodosRepository::new(db_pool)),
        }
    }
}

pub async fn run_server(server_state: ServerState) -> Result<(), anyhow::Error> {
    let app = new_api_router(Arc::new(server_state));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
