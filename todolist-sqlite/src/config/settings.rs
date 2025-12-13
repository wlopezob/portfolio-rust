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
pub struct AppSettings {
    pub server: ServerConfig,
    pub app: AppConfig,
}

impl AppSettings {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("src/properties/application.yaml"))
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