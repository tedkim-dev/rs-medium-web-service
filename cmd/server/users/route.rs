use std::sync::Arc;

use axum::{routing::get, Router};

use crate::server::ServerState;

use super::handlers::{get_user_by_id, get_users};

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user_by_id))
}