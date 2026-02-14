use core::str;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenApiConfig {
    pub ui_path: String,
    pub json_path: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub server: ServerConfig,
    pub app: AppConfig,
    pub openapi: OpenApiConfig,
}

impl AppSettings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let profile = std::env::var("PROFILE").unwrap_or_else(|_| "dev".to_string());
        config::Config::builder()
            .add_source(config::File::with_name("src/properties/application.yaml"))
            .add_source(config::File::with_name(&format!("src/properties/application-{}.yaml", 
                profile)))
            .build()?
            .try_deserialize()
    }

    pub fn server_address(&self) -> String {
        format!(
            "{}:{}",
            self.server.host,
            self.server.port
        )
    }
}