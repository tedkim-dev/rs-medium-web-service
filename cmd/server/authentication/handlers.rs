use std::sync::Arc;

use crate::{errors::ApiError, server::ServerState};
use axum::{extract::State, Json};
use http::HeaderMap;
use jsonwebtoken::{decode, encode, Validation};

use super::{auth_extractor::RequireAuthentication, Claims, LoginRequest, LoginResponse};

pub async fn login(
    State(server_state): State<Arc<ServerState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let email = request.email;
    let password = request.password;

    let user = server_state
        .user_service
        .get_user_by_email(email)
        .await
        .map_err(|e| ApiError::Service(anyhow::anyhow!(e)))?;

    // TODO: bcrypt password
    if user.password != password {
        return Err(ApiError::Unauthorized(anyhow::anyhow!("Invalid password")));
    }

    // generate token
    let claims = Claims {
        sub: user.email.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = match encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(server_state.jwt_secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Failed to encode token: {}", e);
            return Err(ApiError::Service(anyhow::anyhow!(e)));
        }
    };

    Ok(Json(LoginResponse { token }))
}

pub async fn get_info(
    header: HeaderMap,
    State(server_state): State<Arc<ServerState>>,
) -> Result<Json<String>, ApiError> {
    if let Some(auth_header) = header.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str
                    .split_whitespace()
                    .nth(1)
                    .ok_or(ApiError::Unauthorized(anyhow::anyhow!("Invalid token")))?;

                match decode::<Claims>(
                    token,
                    &jsonwebtoken::DecodingKey::from_secret(server_state.jwt_secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => {
                        return Ok(Json(format!("You are valid: {}", token_data.claims.sub)));
                    }
                    Err(e) => {
                        eprintln!("Failed to decode token: {}", e);
                        return Err(ApiError::Unauthorized(anyhow::anyhow!(e)));
                    }
                }
            }
        }
    }

    Err(ApiError::Unauthorized(anyhow::anyhow!("Unauthorized")))
}

pub async fn protected(
    RequireAuthentication(auth_user): RequireAuthentication,
) -> Result<Json<String>, ApiError> {
    Ok(Json(format!(
        "protected: You are valid: {}",
        auth_user.email
    )))
}
