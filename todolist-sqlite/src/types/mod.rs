use std::sync::{Arc, Mutex};

use sqlite::Connection;

use crate::service::todo_service::TodoServiceInterface;


pub type Db = Arc<Mutex<Connection>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub todo_service: Arc<dyn TodoServiceInterface>,
}

impl AppState {
    pub fn new(db: Db, todo_service: Arc<dyn TodoServiceInterface>) -> Self {
        Self { db, todo_service }
    }
}