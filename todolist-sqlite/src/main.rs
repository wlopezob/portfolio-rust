use std::sync::{Arc, Mutex};

use crate::{config::{app_info}, service::todo_service, types::AppState};
mod config;
mod controller;
mod routes;
mod types;
mod model;
mod service;
mod api;

#[tokio::main]
async fn main() { 
    // Load configuration
    let app_settings = config
        ::settings
        ::AppSettings::new().expect("Failed to load app settings");
   
    let app_info = app_info::AppInfo::new();
    let app_state = setup_app_state();
    let app = routes::build_router(&app_settings, app_state);

    api::start_server(app, &app_settings, &app_info).await;
}

fn setup_app_state() -> AppState {
    let connection = sqlite::open("data/todo.db").unwrap();
    let todo_service = Arc::new(todo_service::TodoServiceImpl{});

    AppState::new(
         Arc::new(Mutex::new(connection)),
        todo_service
    )
}