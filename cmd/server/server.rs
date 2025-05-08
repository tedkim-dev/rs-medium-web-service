use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use todo_service::{Repository as TodosRepository, Service as TodosService};
use tokio::sync::watch;
use user_service::UserService;

use crate::router::new_api_router;

#[derive(Clone, FromRef)]
pub struct ServerState {
    pub db_pool: PgPool,
    pub todos_service: TodosService,
    pub user_service: UserService,
    // TODO: secret string
    pub jwt_secret: String,
}

impl ServerState {
    pub fn new(db_pool: PgPool, jwt_secret: String) -> Self {
        Self {
            db_pool: db_pool.clone(),
            todos_service: TodosService::new(TodosRepository::new(db_pool.clone())),
            user_service: UserService::new(db_pool),
            jwt_secret,
        }
    }
}

pub async fn run_server(server_state: ServerState, mut shutdown_rx: watch::Receiver<bool>) {
    let app = new_api_router(Arc::new(server_state));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server: listening on port 3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_rx.changed().await.unwrap();
        })
        .await
        .unwrap();
}
