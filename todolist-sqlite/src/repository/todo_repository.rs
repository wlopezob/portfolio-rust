use async_trait::async_trait;

use crate::{model::{api_exception::ApiException, todo_item_model::TodoItemModel}, types::Db, utils::utils::{to_display_date, to_iso_date}};


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
            .prepare("INSERT INTO todos (title, completed, max_date) VALUES (?, ?, ?)")
            .unwrap();
        statement.bind((1, todo_item.title.as_str())).unwrap();
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))
            .unwrap();
        let iso_date = to_iso_date(&todo_item.max_date);
        statement.bind((3, iso_date.as_str())).unwrap();
        statement.next()?;

        // obtain the last inserted id
        let mut statement = connection.prepare("SELECT last_insert_rowid()").unwrap();
        statement.next()?;

        let response_payload = TodoItemModel {
            id: statement.read::<i64, _>(0)?,
            title: todo_item.title,
            completed: todo_item.completed,
            max_date: to_display_date(&iso_date),
        };
        Ok(response_payload)
    }

    async fn get_all(&self) -> Result<Vec<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;
        let query = "SELECT id, title, completed, max_date FROM todos";

        let mut statement = connection.prepare(query)?;

        let todos: Result<Vec<TodoItemModel>, sqlite::Error> = statement
            .iter()
            .map(|row| {
                let row = row?;
                Ok(TodoItemModel {
                    id: row.read::<i64, _>("id"),
                    title: row.read::<&str, _>("title").to_string(),
                    completed: row.read::<i64, _>("completed") != 0,
                    max_date: to_display_date(row.read::<&str, _>("max_date")),
                })
            })
            .collect();

        Ok(todos?)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;
        let query = "SELECT id, title, completed, max_date FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query)?;
        statement.bind((1, id))?;

        if statement.next()? == sqlite::State::Row {
            Ok(Some(TodoItemModel {
                id: statement.read::<i64, _>("id")?,
                title: statement.read::<String, _>("title")?,
                completed: statement.read::<i64, _>("completed")? != 0,
                max_date: to_display_date(&statement.read::<String, _>("max_date")?),
            }))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, id: i64, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException> {
        let connection = self.db.lock()?;
        let query = "UPDATE todos SET title = ?, completed = ?, max_date = ? WHERE id = ?";

        let mut statement = connection.prepare(query)?;
        statement.bind((1, todo_item.title.as_str()))?;
        statement
            .bind((2, if todo_item.completed { 1 } else { 0 }))?;
        let iso_date = to_iso_date(&todo_item.max_date);
        statement.bind((3, iso_date.as_str()))?;
        statement.bind((4, id))?;
        statement.next()?;

        let response_payload = TodoItemModel {
            id,
            title: todo_item.title,
            completed: todo_item.completed,
            max_date: to_display_date(&iso_date),
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