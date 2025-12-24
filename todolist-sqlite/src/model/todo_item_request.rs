use serde::{Deserialize, Serialize};

/// Request model for creating or updating a todo item
/// 
/// This structure represents the data required to create or update a todo item
#[derive(Serialize, Deserialize, Default, Clone, Debug, 
    utoipa::ToSchema)]
pub struct TodoItemRequest {
    /// Optional identifier (used for updates, omit for creation)
    #[schema(example = 1)]
    pub id: Option<i64>,
    
    /// Title or description of the todo task
    #[schema(example = "Buy groceries")]
    pub title: String,
    
    /// Indicates whether the todo item is completed
    #[schema(example = false)]
    pub completed: bool,
}