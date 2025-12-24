use std::env;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }
    }
}

impl AppInfo {
    pub fn new() -> Self {
        Self::default()
    }
}