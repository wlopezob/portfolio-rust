use crate::{
    model::{todo_item_request::TodoItemRequest},
    types::{AppState},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{StatusCode},
    response::IntoResponse,
    routing::{get, post},
};

pub struct TodoController;

impl TodoController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route(
                "/",
                post(TodoController::create_todo)
                .get(TodoController::get_all),
            )
            .route(
                "/{id}",
                get(TodoController::get_by_id)
                    .put(TodoController::update_todo)
                    .delete(TodoController::delete_by_id),
            )
    }

    pub async fn create_todo(
        State(app_state): State<AppState>,
        Json(payload): Json<TodoItemRequest>,
    ) -> impl IntoResponse {

        app_state
            .todo_service
            .create(&app_state.db, payload)
            .await
            .map(|todo| (StatusCode::CREATED, Json(todo)).into_response())
            .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
    }

    pub async fn get_all(State(app_state): State<AppState>) -> impl IntoResponse {
        app_state
            .todo_service
            .get_all(&app_state.db)
            .await
            .map(|todos| (StatusCode::OK, Json(todos)).into_response())
            .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
    }

    pub async fn get_by_id(
        State(app_state): State<AppState>,
        Path(id): Path<i64>,
    ) -> impl IntoResponse {
        app_state
            .todo_service
            .get_by_id(&app_state.db, id)
            .await
            .map(|todo| (StatusCode::OK, Json(todo)).into_response())
            .unwrap_or_else(|e| (StatusCode::NOT_FOUND, e).into_response())
    }

    pub async fn update_todo(
        State(app_state): State<AppState>,
        Path(id): Path<i64>,
        Json(payload): Json<TodoItemRequest>,
    ) -> impl IntoResponse {
        app_state
            .todo_service
            .update(&app_state.db, id, payload)
            .await
            .map(|todo| (StatusCode::OK, Json(todo)).into_response())
            .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
    }

    pub async fn delete_by_id(
        State(app_state): State<AppState>,
        Path(id): Path<i64>,
    ) -> impl IntoResponse {
        app_state
            .todo_service
            .delete_by_id(&app_state.db, id)
            .await
            .map(|_| StatusCode::NO_CONTENT.into_response())
            .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
    }
}