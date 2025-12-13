use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TodoItemResponse {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}