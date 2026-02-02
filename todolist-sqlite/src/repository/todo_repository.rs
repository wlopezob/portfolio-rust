use async_trait::async_trait;

use crate::{model::{api_exception::ApiException, todo_item_model::TodoItemModel}, types::Db};


#[async_trait]
pub trait TodoRepositoryInterface: Send + Sync {
    async fn create(&self, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException>;
    async fn get_all(&self) -> Result<Vec<TodoItemModel>, ApiException>;
    async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException>;
    async fn update(&self, id: i64, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException>;
    async fn delete_by_id(&self, id: i64) -> Result<(), ApiException>;
}

pub struct TodoRepositoryImpl {
    db: Db
}

impl TodoRepositoryImpl {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepositoryInterface for TodoRepositoryImpl {
    async fn create(&self, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException> {
        let connection = self.db.lock()?;
        let mut statement = connection
            .prepare("INSERT INTO todos (title, completed) VALUES (?, ?)")
            .unwrap();
        statement.bind((1, todo_item.title.as_str())).unwrap();
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))
            .unwrap();
        statement.next()?;

        // obtain the last inserted id
        let mut statement = connection.prepare("SELECT last_insert_rowid()").unwrap();
        statement.next()?;

        let response_payload = TodoItemModel {
            id: statement.read::<i64, _>(0)?,
            title: todo_item.title,
            completed: todo_item.completed,
        };
        Ok(response_payload)
    }

    async fn get_all(&self) -> Result<Vec<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;
        let query = "SELECT id, title, completed FROM todos";

        let mut statement = connection.prepare(query)?;

        let todos: Result<Vec<TodoItemModel>, sqlite::Error> = statement
            .iter()
            .map(|row| {
                let row = row?;
                Ok(TodoItemModel {
                    id: row.read::<i64, _>("id"),
                    title: row.read::<&str, _>("title").to_string(),
                    completed: row.read::<i64, _>("completed") != 0,
                })
            })
            .collect();

        Ok(todos?)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;
        let query = "SELECT id, title, completed FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query)?;
        statement.bind((1, id))?;

        if statement.next()? == sqlite::State::Row {
            Ok(Some(TodoItemModel {
                id: statement.read::<i64, _>("id")?,
                title: statement.read::<String, _>("title")?,
                completed: statement.read::<i64, _>("completed")? != 0,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, id: i64, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException> {
        let connection = self.db.lock()?;
        let query = "UPDATE todos SET title = ?, completed = ? WHERE id = ?";

        let mut statement = connection.prepare(query)?;
        statement.bind((1, todo_item.title.as_str()))?;
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))?;
        statement.bind((3, id))?;
        statement.next()?;

        let response_payload = TodoItemModel {
            id,
            title: todo_item.title,
            completed: todo_item.completed,
        };
        Ok(response_payload)
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), ApiException> {
        let connection = self.db.lock()?;
        let query = "DELETE FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query)?;
        statement.bind((1, id))?;
        statement.next()?;

        Ok(())
    }
}