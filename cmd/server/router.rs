use std::sync::Arc;

use axum::{response::IntoResponse, Json, Router};
use http::{HeaderValue, Method, StatusCode, Uri};

use crate::{authentication, server::ServerState, todos, users};

pub fn new_api_router(
    server_state: Arc<ServerState>,
) -> Router {
    let todos_router = todos::router();
    let users_router = users::router();
    let auth_router = authentication::router();

    axum::Router::new()
        .route("/", axum::routing::get(handler))
        .merge(auth_router)
        .merge(todos_router)
        .merge(users_router)
        .fallback(fallback_handler)
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

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}
