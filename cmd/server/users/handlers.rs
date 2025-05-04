use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::types::Uuid;
use user_service::User;

use crate::{errors::ApiError, server::ServerState};

pub async fn get_users(
    State(server_state): State<Arc<ServerState>>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = server_state
        .user_service
        .get_users()
        .await
        .map_err(|e| match e {
            user_service::Error::GetUsersFailed(sqlx::Error::RowNotFound) => {
                ApiError::NotFound(anyhow::anyhow!(e))
            }
            _ => ApiError::Service(anyhow::anyhow!(e)),
        })?;
    Ok(Json(users))
}

pub async fn get_user_by_id(
    State(server_state): State<Arc<ServerState>>,
    Path(id): Path<String>,
) -> Result<Json<User>, ApiError> {
    let id =
        Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest(anyhow::anyhow!("Invalid id")))?;

    let user = server_state
        .user_service
        .get_user_by_id(id)
        .await
        .map_err(|e| match e {
            user_service::Error::GetUserByIdFailed(sqlx::Error::RowNotFound) => {
                ApiError::NotFound(anyhow::anyhow!(e))
            }
            _ => ApiError::Service(anyhow::anyhow!(e)),
        })?;
    Ok(Json(user))
}
