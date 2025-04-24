use crate::ServerState;
use axum::{extract::State, Json};
use todo_service::Todo;

pub async fn get_todos(State(server_state): State<ServerState>) -> Json<Vec<Todo>> {
    let todos = server_state.todos_service.get_todos().await.unwrap();
    Json(todos)
}
