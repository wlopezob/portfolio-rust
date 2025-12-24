use crate::{
    config::open_api::TAG_TODO,
    model::{todo_item_request::TodoItemRequest, todo_item_response::TodoItemResponse},
    types::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

pub struct TodoController;
impl TodoController {
    pub fn router() -> OpenApiRouter<AppState> {
        OpenApiRouter::new()
            // Rutas para la raíz "/"
            .routes(routes!(create_todo))
            .routes(routes!(get_all))
            // Rutas para "/{id}"
            .routes(routes!(get_by_id))
            .routes(routes!(update_todo))
            .routes(routes!(delete_by_id))
    }
}

#[utoipa::path(
        post,
        path = "/",
        tag = TAG_TODO,
        request_body = TodoItemRequest,
        responses(
            (status = 201, description = "Todo item created successfully", body = TodoItemResponse),
            (status = 500, description = "Internal server error")
        )
    )]
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

#[utoipa::path(
        get,
        path = "/",
        tag = TAG_TODO,
        responses(
            (status = 200, description = "List of todo items", body = [TodoItemResponse]),
            (status = 500, description = "Internal server error")
        )
    )]
pub async fn get_all(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state
        .todo_service
        .get_all(&app_state.db)
        .await
        .map(|todos| (StatusCode::OK, Json(todos)).into_response())
        .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
}

#[utoipa::path(
        put,
        path = "/{id}",
        tag = TAG_TODO,
        request_body = TodoItemRequest,
        responses(
            (status = 200, description = "Todo item updated successfully", body = TodoItemResponse),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = i64, Path, description = "ID of the todo item to update")
        )
    )]
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

#[utoipa::path(
        get,
        path = "/{id}",
        tag = TAG_TODO,
        responses(
            (status = 200, description = "Todo item found", body = TodoItemResponse),
            (status = 404, description = "Todo item not found")
        ),
        params(
            ("id" = i64, Path, description = "ID of the todo item to retrieve")
        )
    )]
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

#[utoipa::path(
        delete,
        path = "/{id}",
        tag = TAG_TODO,
        responses(
            (status = 204, description = "Todo item deleted successfully"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = i64, Path, description = "ID of the todo item to delete")
        )
    )]
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
