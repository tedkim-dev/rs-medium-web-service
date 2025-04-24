use axum::{extract::FromRef, Json};
use http::{HeaderValue, Method};
use sqlx::PgPool;

use todo_service::{Service as TodosService, Repository as TodosRepository};

use crate::todos;

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
    let todos_router = todos::router();

    let app = axum::Router::new()
        .route("/", axum::routing::get(handler))
        .merge(todos_router)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .with_state(server_state);

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}
