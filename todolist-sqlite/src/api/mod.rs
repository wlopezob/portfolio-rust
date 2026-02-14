use axum::Router;

use crate::config::{app_info::AppInfo, settings::AppSettings};

pub async fn start_server(app: Router, app_settings: &AppSettings, app_info: &AppInfo) {
    let address = app_settings.server_address();

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(&address)
            .await
            .unwrap();

    print_startup_banner(&address, app_info, &app_settings);

    axum::serve(listener, app).await.unwrap();
    
    

}

fn print_startup_banner(address: &str, app_info: &AppInfo, app_settings: &AppSettings) {
    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║  🚀 {} v{}", app_info.name, app_info.version);
    println!("║  📝 {}", app_info.description);
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║  🌐 Server:  http://{}", address);
    if app_settings.openapi.enabled {
        println!("║  📚 Swagger: http://{}{}/swagger-ui", address, app_settings.app.prefix);
        println!("║  📄 OpenAPI: http://{}{}/api-docs/openapi.json", address, app_settings.app.prefix);
    }
    println!("║  🔗 API:     http://{}{}/todo", address, app_settings.app.prefix);
    println!("╚═══════════════════════════════════════════════════╝\n");
}
