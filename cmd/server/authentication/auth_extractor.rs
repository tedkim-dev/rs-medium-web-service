use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, Validation};

use crate::{errors::ApiError, server::ServerState};

use super::{AuthUser, Claims};

pub struct RequireAuthentication(pub AuthUser);

impl<S> FromRequestParts<S> for RequireAuthentication
where
    Arc<ServerState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|token| token.strip_prefix("Bearer "));

        if let Some(token) = auth_header {
            let server_state = Arc::<ServerState>::from_ref(state);
            match decode::<Claims>(
                token,
                &jsonwebtoken::DecodingKey::from_secret(server_state.jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(token_data) => Ok(Self(AuthUser {
                    email: token_data.claims.sub,
                })),
                Err(e) => {
                    eprintln!("Failed to decode token: {}", e);
                    Err(ApiError::Unauthorized(anyhow::anyhow!(e)))
                }
            }
        } else {
            Err(ApiError::Unauthorized(anyhow::anyhow!("Unauthorized")))
        }
    }
}
