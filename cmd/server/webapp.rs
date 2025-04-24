use std::sync::Arc;

use axum::{Json, Router};
use http::{HeaderValue, Method};

use crate::{server::ServerState, todos};

pub fn new_api_router(server_state: Arc<ServerState>) -> Router {
    let todos_router = todos::router();

    axum::Router::new()
        .route("/", axum::routing::get(handler))
        .merge(todos_router)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .with_state(server_state)
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
