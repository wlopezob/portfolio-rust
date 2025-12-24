use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response model for a todo item
/// 
/// This structure represents a todo item returned by the API
#[derive(Serialize, Deserialize, Default, Clone, Debug, ToSchema)]
pub struct TodoItemResponse {
    /// Unique identifier of the todo item
    #[schema(example = 1, rename = "id", additional_properties = false)]
    pub id: Option<i64>,
    
    /// Title or description of the todo task
    #[schema(example = "Buy groceries")]
    pub title: String,
    
    /// Indicates whether the todo item has been completed
    #[schema(example = false)]
    pub completed: bool,
}