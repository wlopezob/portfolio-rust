use std::net::SocketAddr;

use anyhow::Result;
use rmcp::transport::{StreamableHttpService, streamable_http_server::session::local::LocalSessionManager};

mod auth;
mod filters;
mod formatter;
mod gcloud;
mod models;
mod request;
mod server;
mod time;

use auth::GcloudAuthProvider;
use server::CloudLoggingService;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8766".parse()?;

    // Create authentication provider
    let auth_provider = GcloudAuthProvider::new();
    
    // Create the MCP service
    let logging_service = CloudLoggingService::new(auth_provider);

    let service = StreamableHttpService::new(
        move || Ok(logging_service.clone()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new()
        .nest_service("/mcp", service);

    println!("Google Cloud Logging MCP server listening on {}", addr);
    println!("Endpoint: http://{}/mcp", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl-c")
        })
        .await?;

    Ok(())
}
