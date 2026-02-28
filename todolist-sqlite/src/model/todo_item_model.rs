
#[derive(Debug, Clone)]
pub struct TodoItemModel {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub max_date: String,
}