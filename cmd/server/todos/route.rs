use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{server::ServerState, todos::handlers::get_todos};

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/todos", get(get_todos))
}
