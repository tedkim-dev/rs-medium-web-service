use std::sync::Arc;

use axum::{routing::{post, get}, Router};

use crate::server::ServerState;

use super::handlers::{get_info, login};

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/login", post(login))
        .route("/info", get(get_info))
}
