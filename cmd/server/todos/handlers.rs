use std::sync::Arc;

use crate::{errors::ApiError, ServerState};
use axum::{extract::State, Json};
use todo_service::Todo;

pub async fn get_todos(
    State(server_state): State<Arc<ServerState>>,
) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = server_state
        .todos_service
        .get_todos()
        .await
        .map_err(|e| {
            match e {
                todo_service::Error::GetTodosFailed(sqlx::Error::RowNotFound) => {
                    ApiError::NotFound(anyhow::anyhow!(e))
                }
                _ => ApiError::Service(anyhow::anyhow!(e)),
            }
        })?;

    Ok(Json(todos))
}
