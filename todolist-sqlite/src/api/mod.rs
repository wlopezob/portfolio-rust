use axum::Router;

use crate::config::{app_info::AppInfo, settings::AppSettings};

pub async fn start_server(app: Router, app_settings: &AppSettings, app_info: &AppInfo) {
    let address = app_settings.server_address();

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(&address)
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
    
    print_startup_banner(&address, app_info, &app_settings.app.prefix);

}

fn print_startup_banner(address: &str, app_info: &AppInfo, prefix: &str) {
    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║  🚀 {} v{}", app_info.name, app_info.version);
    println!("║  📝 {}", app_info.description);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  🌐 Server:  http://{}", address);
    println!("║  📚 Swagger: http://{}{}/swagger-ui", address, prefix);
    println!("║  📄 OpenAPI: http://{}{}/api-docs/openapi.json", address, prefix);
    println!("║  🔗 API:     http://{}{}/todo", address, prefix);
    println!("╚═══════════════════════════════════════════════════╝\n");
}
