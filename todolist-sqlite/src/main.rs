use std::sync::{Arc, Mutex};

use axum::Router;

use crate::{service::todo_service, types::{AppState}};
mod config;
mod controller;
mod routes;
mod types;
mod model;
mod service;

#[tokio::main]
async fn main() { 
    // Load configuration
    let app_settings = config
        ::settings
        ::AppSettings::new().expect("Failed to load app settings");
   
    let connection = sqlite::open("data/todo.db").unwrap();
    
    let todo_service = Arc::new(todo_service::TodoServiceImpl{});

    let app_state = AppState::new(
         Arc::new(Mutex::new(connection)),
        todo_service
    );

    let app = Router::new()
        .nest(&app_settings.app.prefix, routes::create_routes())
        .with_state(app_state);

    let listener = tokio::net
        ::TcpListener
        ::bind(app_settings.server_address()).await.unwrap();
    
    println!("Server running at http://{}", app_settings.server_address());

    axum::serve(listener, app).await.unwrap()
}
