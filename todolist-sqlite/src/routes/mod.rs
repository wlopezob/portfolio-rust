use axum::Router;

use crate::{controller::todo_controller::TodoController, types::{AppState}};

pub fn create_routes() -> Router<AppState> {
    axum::Router::new()
        .nest("/todo", TodoController::routes())
}