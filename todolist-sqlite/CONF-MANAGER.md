# Configuration Manager - Implementation Guide

This document explains how application configuration is managed in this Rust/Axum project using the `config` crate with YAML files and environment-based profiles.

## 📋 Table of Contents

- [Overview](#overview)
- [Dependencies](#dependencies)
- [Configuration Structure](#configuration-structure)
- [Implementation](#implementation)
- [Profile System](#profile-system)
- [Environment Variables with dotenvy](#environment-variables-with-dotenvy)
- [Best Practices](#best-practices)
- [Summary](#summary)

## Overview

This project uses a **multi-environment configuration system** that:

- ✅ Separates configuration by environment (dev, staging, production)
- ✅ Uses YAML files for easy readability and maintenance
- ✅ Supports configuration merging (base + profile-specific)
- ✅ Type-safe configuration with Rust structs
- ✅ Environment variable control via `PROFILE`
- ✅ Automatic `.env` file loading with `dotenvy`

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
config = "0.15.19"
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
```

**Dependencies Explained:**
- **`config`**: Hierarchical configuration management with YAML support
- **`dotenvy`**: Loads environment variables from `.env` files
- **`serde`**: Serialization/deserialization for configuration structs

## Configuration Structure

### Directory Layout

```
src/
└── properties/
    ├── application.yaml         # Base configuration (shared across all profiles)
    ├── application-dev.yaml     # Development overrides
    ├── application-sta.yaml     # Staging overrides
    └── application-prod.yaml    # Production overrides
```

### Configuration Files

**`application.yaml`** (Base Configuration):
```yaml
server:
  host: "0.0.0.0"
  port: 8080

app:
  prefix: "/api"

openapi:
  ui_path: "/swagger-ui"
  json_path: "/api-docs/openapi.json"
  enabled: true
```

**`application-dev.yaml`** (Development Profile):
```yaml
server:
  host: "0.0.0.0"
  port: 8080

app:
  prefix: "/api"

openapi:
  ui_path: "/swagger-ui"
  json_path: "/api-docs/openapi.json"
  enabled: true  # ✅ Swagger enabled in dev
```

**`application-prod.yaml`** (Production Profile):
```yaml
server:
  host: "0.0.0.0"
  port: 8080

app:
  prefix: "/api"

openapi:
  ui_path: "/swagger-ui"
  json_path: "/api-docs/openapi.json"
  enabled: false  # ❌ Swagger disabled in production
```

## Implementation

### Step 1: Define Configuration Structs

**File**: `src/config/settings.rs`

```rust
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
        // 1. Get profile from environment variable (default: "dev")
        let profile = std::env::var("PROFILE")
            .unwrap_or_else(|_| "dev".to_string());
        
        // 2. Build configuration by merging files
        config::Config::builder()
            .add_source(config::File::with_name("src/properties/application.yaml"))
            .add_source(config::File::with_name(&format!(
                "src/properties/application-{}.yaml", 
                profile
            )))
            .build()?
            .try_deserialize()
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
```

## Profile System

### How It Works

1. **Environment Variable**: The `PROFILE` environment variable controls which configuration to load
2. **Fallback**: If `PROFILE` is not set, it defaults to `"dev"`
3. **Merge Strategy**: 
   - First loads `application.yaml` (base configuration)
   - Then overlays `application-{profile}.yaml` (profile-specific overrides)
   - Profile-specific values override base values

### Configuration Merge Example

**Base (`application.yaml`):**
```yaml
server:
  host: "0.0.0.0"
  port: 8080

openapi:
  enabled: true
```

**Profile (`application-prod.yaml`):**
```yaml
openapi:
  enabled: false  # This overrides the base value
```

**Resulting Configuration in Production:**
```yaml
server:
  host: "0.0.0.0"  # From base
  port: 8080        # From base

openapi:
  enabled: false    # From profile (overridden)
```

### Setting the Profile

**Development (default):**
```bash
cargo run
# Uses application-dev.yaml
```

**Staging:**
```bash
PROFILE=sta cargo run
# Uses application-sta.yaml
```

**Production:**
```bash
PROFILE=prod cargo run
# Uses application-prod.yaml
```

**Using .env file (recommended):**
```bash
# Create .env file in project root
echo "PROFILE=dev" > .env
cargo run
```

## Environment Variables with dotenvy

This project uses `dotenvy` to automatically load environment variables from `.env` files, making local development easier and more consistent.

### Why dotenvy?

- ✅ **Automatic loading**: No need to manually export variables
- ✅ **Development friendly**: Different `.env` files for different environments
- ✅ **Security**: `.env` files can be gitignored to protect sensitive data
- ✅ **Consistency**: Same configuration approach across team members
- ✅ **Simple integration**: One line of code to enable

### Setup

**1. Add dependency:**
```toml
[dependencies]
dotenvy = "0.15"
```

**2. Load environment variables in `main.rs`:**

```rust
#[tokio::main]
async fn main() { 
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Now environment variables are available
    let app_settings = config::settings::AppSettings::new()
        .expect("Failed to load application settings");
    
    // Rest of your application...
}
```

**Why `.ok()`?**
- `dotenv()` returns `Result<PathBuf, Error>`
- `.ok()` converts it to `Option<PathBuf>`, ignoring errors
- This allows the app to run even if `.env` file doesn't exist (useful for production where you use actual environment variables)

### Environment File Structure

**`.env`** (Development - Local):
```bash
# Profile selection
PROFILE=dev
```

**`.env.example`** (Template - Committed to Git):
```bash
# Copy this file to .env and customize
PROFILE=dev
```

**`.gitignore`**:
```
# Environment files (keep secrets safe)
.env
```

### Usage Patterns

#### Pattern 1: Simple Profile Switching

**Development:**
```bash
# .env
PROFILE=dev
```

**Staging:**
```bash
# .env
PROFILE=sta
```

**Production (no .env file needed):**
```bash
# Set in deployment environment
export PROFILE=prod
cargo run
```

### Integration with Config System

The `dotenvy` crate works seamlessly with the `config` crate:

```rust
impl AppSettings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // 1. dotenvy loads PROFILE from .env file
        // 2. std::env::var reads it from environment
        let profile = std::env::var("PROFILE")
            .unwrap_or_else(|_| "dev".to_string());
        
        // 3. config crate loads appropriate YAML files
        config::Config::builder()
            .add_source(config::File::with_name("src/properties/application.yaml"))
            .add_source(config::File::with_name(&format!(
                "src/properties/application-{}.yaml", 
                profile
            )))
            .build()?
            .try_deserialize()
    }
}
```

**Flow Diagram:**
```
.env file (PROFILE=dev)
    ↓
dotenvy::dotenv() loads into environment
    ↓
std::env::var("PROFILE") reads "dev"
    ↓
Loads application.yaml + application-dev.yaml
    ↓
Deserializes into AppSettings struct
```

### Best Practices with dotenvy

#### ✅ DO

**1. Always provide `.env.example`:**
```bash
# .env.example (commit this)
PROFILE=dev
# DATABASE_URL=sqlite://./data/todos.db
# LOG_LEVEL=info
```

**2. Use `.ok()` for optional loading:**
```rust
// ✅ Good - doesn't crash if file missing
dotenvy::dotenv().ok();

// ❌ Bad - crashes if file missing
dotenvy::dotenv().unwrap();
```

**3. Load early in main():**
```rust
#[tokio::main]
async fn main() {
    // ✅ Load first, before any configuration
    dotenvy::dotenv().ok();
    
    let config = load_config();
    // ...
}
```



## Best Practices

### ✅ DO

**1. Use descriptive struct names:**
```rust
// ✅ Good
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

// ❌ Bad
pub struct Server {
    pub h: String,
    pub p: u16,
}
```

**2. Group related configuration:**
```rust
// ✅ Good - grouped by feature
pub struct AppSettings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub openapi: OpenApiConfig,
}

// ❌ Bad - flat structure
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub db_path: String,
    pub swagger_enabled: bool,
}
```

## Summary

This configuration management system provides:

1. **Separation of Concerns**: Different settings for different environments
2. **Type Safety**: Rust structs ensure configuration validity at compile time
3. **Flexibility**: Easy to add new configuration sections
4. **Maintainability**: YAML files are human-readable and easy to edit
5. **Security**: Sensitive data can be excluded from production builds
6. **Developer Experience**: `.env` files for easy local development

### Key Takeaways

- Use `config` crate for hierarchical configuration management
- Use `dotenvy` crate for automatic `.env` file loading
- Organize settings into logical groups (structs)
- Leverage the profile system for environment-specific settings
- Always provide sensible defaults with `unwrap_or_else()`
- Always use `.ok()` with `dotenvy::dotenv()` for graceful fallback
- Document configuration behavior, especially security-related settings
- Never commit sensitive data to configuration files (use `.gitignore`)
- Provide `.env.example` as a template for new developers

### Configuration Flow

```
1. dotenvy loads .env file → environment variables
2. config reads PROFILE from environment
3. config loads application.yaml (base)
4. config loads application-{profile}.yaml (overrides)
5. config deserializes into AppSettings struct
6. Application uses type-safe configuration
```

For more information:
- `config` crate: https://docs.rs/config/latest/config/
- `dotenvy` crate: https://docs.rs/dotenvy/latest/dotenvy/
