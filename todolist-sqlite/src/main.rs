use std::sync::{Arc, Mutex};

use crate::{config::app_info, repository::todo_repository::{TodoRepositoryImpl, TodoRepositoryInterface}, service::todo_service::{self, TodoServiceImpl, TodoServiceInterface}, types::AppState};
mod config;
mod controller;
mod routes;
mod types;
mod model;
mod service;
mod api;
mod repository;
mod utils;
mod validators;

#[tokio::main]
async fn main() { 
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Load configuration
    let app_settings = config
        ::settings
        ::AppSettings::new().expect("Failed to load app settings");
   
    let app_info = app_info::AppInfo::new();
    let app_state = setup_app_state();
    let app = routes::build_router(&app_settings, &app_info, app_state);

    api::start_server(app, &app_settings, &app_info).await;
}

fn setup_database(connection: &sqlite::Connection) {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            max_date DATE NOT NULL
        )"
    ).unwrap();
}

fn setup_app_state() -> AppState {
    let connection = sqlite::open("data/todo.db").unwrap();
    setup_database(&connection);
    let db = Arc::new(Mutex::new(connection));
    let todo_repository: Arc<dyn TodoRepositoryInterface> = Arc::new(TodoRepositoryImpl::new(db.clone()));
    let todo_service: Arc<dyn TodoServiceInterface> = Arc::new(TodoServiceImpl::new(todo_repository));

    AppState::new(
        db,
        todo_service
    )
}