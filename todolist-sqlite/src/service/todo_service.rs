use std::sync::Arc;

use crate::{
    model::{api_exception::ApiException, todo_item_model::TodoItemModel, todo_item_request::TodoItemRequest, todo_item_response::{self, TodoItemResponse}}, repository::todo_repository::TodoRepositoryInterface, types::Db
};

#[async_trait::async_trait]
pub trait TodoServiceInterface: Send + Sync {
    async fn create(&self, todo_item: TodoItemRequest)
    -> Result<TodoItemResponse, ApiException>;
    async fn get_all(&self) -> Result<Vec<TodoItemResponse>, ApiException>;
    async fn get_by_id(&self, id: i64) -> Result<TodoItemResponse, ApiException>;
    async fn update(&self, id: i64, todo_item: TodoItemRequest) -> Result<TodoItemResponse, ApiException>;
    async fn delete_by_id(&self, id: i64) -> Result<(), ApiException>;
}

pub struct TodoServiceImpl {
    todo_repository: Arc<dyn TodoRepositoryInterface>,
}

impl TodoServiceImpl {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryInterface>) -> Self {
        Self { todo_repository }
    }
}

#[async_trait::async_trait]
impl TodoServiceInterface for TodoServiceImpl {
    async fn create(
        &self,
        todo_item: TodoItemRequest,
    ) -> Result<TodoItemResponse, ApiException> {
        let todo_item = TodoItemModel {
            id: 0,
            title: todo_item.title,
            completed: todo_item.completed,
        };
        let todo_item_response = self.todo_repository.create(todo_item).await?;
        
        let response_payload = TodoItemResponse {
            id: Some(todo_item_response.id),
            title: todo_item_response.title,
            completed: todo_item_response.completed,
        };

        Ok(response_payload)
    }

    async fn get_all(&self) -> Result<Vec<TodoItemResponse>, ApiException> {
        let item_responses = self.todo_repository.get_all().await?;
        let todos = item_responses
            .iter()
            .map(|row| TodoItemResponse {
                id: Some(row.id),
                title: row.title.clone(),
                completed: row.completed,
            })
            .collect::<Vec<TodoItemResponse>>();
        Ok(todos)
    }

    async fn get_by_id(&self, id: i64) -> Result<TodoItemResponse, ApiException> {
        self.todo_repository.get_by_id(id).await?
            .ok_or_else(|| ApiException::NotFound(format!("Todo item with {} not found", id)))
            .map(|row| TodoItemResponse {
                id: Some(row.id),
                title: row.title,
                completed: row.completed,
            })
    }

    async fn update(&self, id: i64, todo_item: TodoItemRequest) -> Result<TodoItemResponse, ApiException> {
        let todo_item = TodoItemModel {
            id,
            title: todo_item.title,
            completed: todo_item.completed,
        };
        let todo_item_response =self.todo_repository.update(id, todo_item).await?;
        let response_payload = TodoItemResponse {
            id: Some(todo_item_response.id),
            title: todo_item_response.title,
            completed: todo_item_response.completed,
        };
        Ok(response_payload)
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), ApiException> {
        let todo_item = self.todo_repository.get_by_id(id).await?
            .ok_or_else(|| ApiException::NotFound(format!("Todo item with {} not found", id)))?;
        if todo_item.completed {
            return Err(ApiException::BadRequest("Cannot delete a completed todo item".to_string()));
        }
        self.todo_repository.delete_by_id(id).await?;
        Ok(())
    }
}
