use crate::{
    model::{todo_item_request::TodoItemRequest, todo_item_response::TodoItemResponse},
    types::Db,
};

#[async_trait::async_trait]
pub trait TodoServiceInterface: Send + Sync {
    async fn create(&self, db: &Db, todo_item: TodoItemRequest)
    -> Result<TodoItemResponse, String>;
    async fn get_all(&self, db: &Db) -> Result<Vec<TodoItemResponse>, String>;
    async fn get_by_id(&self, db: &Db, id: i64) -> Result<TodoItemResponse, String>;
    async fn update(&self, db: &Db, id: i64, todo_item: TodoItemRequest) -> Result<TodoItemResponse, String>;
    async fn delete_by_id(&self, db: &Db, id: i64) -> Result<(), String>;
}

pub struct TodoServiceImpl;

#[async_trait::async_trait]
impl TodoServiceInterface for TodoServiceImpl {
    async fn create(
        &self,
        db: &Db,
        todo_item: TodoItemRequest,
    ) -> Result<TodoItemResponse, String> {
        let connection = db.lock().unwrap();
        let mut statement = connection
            .prepare("INSERT INTO todos (title, completed) VALUES (?, ?)")
            .unwrap();
        statement.bind((1, todo_item.title.as_str())).unwrap();
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))
            .unwrap();
        statement.next().unwrap();

        // obtain the last inserted id
        let mut statement = connection.prepare("SELECT last_insert_rowid()").unwrap();
        statement.next().unwrap();

        let response_payload = TodoItemResponse {
            id: Option::Some(statement.read::<i64, _>(0).unwrap()),
            title: todo_item.title,
            completed: todo_item.completed,
        };

        Ok(response_payload)
    }

    async fn get_all(&self, db: &Db) -> Result<Vec<TodoItemResponse>, String> {
        let connection = db.lock().unwrap();
        let query = "SELECT id, title, completed FROM todos";

        let mut statement = connection.prepare(query).unwrap();
        statement.next().unwrap();

        let todos = statement
            .iter()
            .map(|row| row.unwrap())
            .map(|row| TodoItemResponse {
                id: Option::Some(row.read::<i64, _>("id")),
                title: row.read::<&str, _>("title").to_string(),
                completed: row.read::<i64, _>("completed") != 0,
            })
            .collect::<Vec<TodoItemResponse>>();
        Ok(todos)
    }

    async fn get_by_id(&self, db: &Db, id: i64) -> Result<TodoItemResponse, String> {
        let connection = db.lock().unwrap();
        let query = "SELECT id, title, completed FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, id)).unwrap();

        let todo_item = statement
            .iter()
            .map(|row| row.unwrap())
            .map(|row| TodoItemResponse {
                id: Option::Some(row.read::<i64, _>("id")),
                title: row.read::<&str, _>("title").to_string(),
                completed: row.read::<i64, _>("completed") != 0,
            })
            .last();

        todo_item.ok_or_else(|| format!("Todo item with id {} not found", id))
    }

    async fn update(&self, db: &Db, id: i64, todo_item: TodoItemRequest) -> Result<TodoItemResponse, String> {
        let connection = db.lock().unwrap();
        let query = "UPDATE todos SET title = ?, completed = ? WHERE id = ?";

        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, todo_item.title.as_str())).unwrap();
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))
            .unwrap();
        statement.bind((3, id)).unwrap();
        statement.next().unwrap();

        let rs = TodoItemResponse {
            id: Some(id),
            title: todo_item.title,
            completed: todo_item.completed,
        };

        Ok(rs)
    }

    async fn delete_by_id(&self, db: &Db, id: i64) -> Result<(), String> {
        let connection = db.lock().unwrap();
        let query = "DELETE FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, id)).unwrap();
        statement.next().unwrap();

        Ok(())
    }
}
