use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TodoItemRequest {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}