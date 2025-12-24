# OpenAPI & Swagger UI Integration Guide

This document explains how OpenAPI and Swagger UI were integrated into this Rust/Axum project.

## 📋 Table of Contents

- [Overview](#overview)
- [Dependencies](#dependencies)
- [Architecture](#architecture)
- [Implementation Steps](#implementation-steps)
- [Configuration](#configuration)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)

## Overview

This project uses `utoipa` and `utoipa-axum` for automatic OpenAPI documentation generation, providing:

- **Automatic API documentation** from Rust code
- **Interactive Swagger UI** for testing endpoints
- **Type-safe documentation** synchronized with code
- **Zero runtime overhead** (compile-time generation)

## Dependencies

```toml
[dependencies]
utoipa = { version = "5.4.0", features = ["axum_extras"] }
utoipa-axum = "0.2.0"
utoipa-swagger-ui = { version = "9.0.2", features = ["axum"] }
```

### What each crate does:

| Crate | Purpose |
|-------|---------|
| `utoipa` | Core OpenAPI generation from Rust types and macros |
| `utoipa-axum` | Axum-specific helpers and auto-route registration |
| `utoipa-swagger-ui` | Embedded Swagger UI for interactive documentation |

## Architecture

```
┌─────────────────────────────────────────────────┐
│              Swagger UI                         │
│         (Interactive Interface)                 │
└─────────────────────┬───────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────┐
│          OpenAPI Specification                  │
│            (JSON/YAML)                          │
└─────────────────────┬───────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────┐
│            utoipa Macros                        │
│   (#[utoipa::path], #[derive(ToSchema)])       │
└─────────────────────┬───────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────┐
│          Rust Code (Controllers)                │
│      (Handlers with annotations)                │
└─────────────────────────────────────────────────┘
```

## Implementation Steps

### 1. Model Annotations

Add `ToSchema` derive to your DTOs:

**TodoItemRequest** (`src/model/todo_item_request.rs`):

```rust
use serde::{Deserialize, Serialize};

/// Request model for creating or updating a todo item
/// 
/// This structure represents the data required to create or update a todo item
#[derive(Serialize, Deserialize, Default, Clone, Debug, 
    utoipa::ToSchema)]
pub struct TodoItemRequest {
    /// Optional identifier (used for updates, omit for creation)
    #[schema(example = 1)]
    pub id: Option<i64>,
    
    /// Title or description of the todo task
    #[schema(example = "Buy groceries")]
    pub title: String,
    
    /// Indicates whether the todo item is completed
    #[schema(example = false)]
    pub completed: bool,
}
```

**TodoItemResponse** (`src/model/todo_item_response.rs`):

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response model for a todo item
/// 
/// This structure represents a todo item returned by the API
#[derive(Serialize, Deserialize, Default, Clone, Debug, ToSchema)]
pub struct TodoItemResponse {
    /// Unique identifier of the todo item
    #[schema(example = 1, rename = "id", additional_properties = false)]
    pub id: Option<i64>,
    
    /// Title or description of the todo task
    #[schema(example = "Buy groceries")]
    pub title: String,
    
    /// Indicates whether the todo item has been completed
    #[schema(example = false)]
    pub completed: bool,
}
```

**Key points:**
- `#[derive(ToSchema)]` o `utoipa::ToSchema` - Genera el schema OpenAPI
- `/// Comentarios` - Descripciones que aparecen en Swagger UI
- `#[schema(example = "...")]` - Valores de ejemplo para Swagger UI

**Cómo agregar descripciones:**

1. **Descripción del struct**: Usa comentarios `///` antes del struct
   ```rust
   /// Response model for a todo item
   /// 
   /// This structure represents a todo item returned by the API
   #[derive(..., ToSchema)]
   pub struct TodoItemResponse {
   ```

2. **Descripción de campos**: Usa comentarios `///` antes de cada campo
   ```rust
   /// Unique identifier of the todo item
   #[schema(example = 1)]
   pub id: Option<i64>,
   ```

3. **Atributos adicionales en `#[schema()]`:**
   - `example = valor` - Valor de ejemplo
   - `rename = "nombre"` - Renombra el campo en el schema
   - `additional_properties = false` - Desactiva propiedades adicionales

### 2. Controller Handlers

Move handlers **outside** the `impl` block and annotate with `#[utoipa::path]`:

```rust
use utoipa_axum::{router::OpenApiRouter, routes};

pub struct TodoController;

impl TodoController {
    pub fn router() -> OpenApiRouter<AppState> {
        OpenApiRouter::new()
            .routes(routes!(
                create_todo,
                get_all,
                get_by_id,
                update_todo,
                delete_by_id
            ))
    }
}

// ⚠️ Handlers MUST be outside impl block when using utoipa-axum

/// Create a new todo item
#[utoipa::path(
    post,
    path = "/",
    request_body = TodoItemRequest,
    responses(
        (status = 201, description = "Todo created successfully", body = TodoItemResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
pub async fn create_todo(
    State(app_state): State<AppState>,
    Json(payload): Json<TodoItemRequest>,
) -> impl IntoResponse {
    app_state
        .todo_service
        .create(&app_state.db, payload)
        .await
        .map(|todo| (StatusCode::CREATED, Json(todo)).into_response())
        .unwrap_or_else(|e| (StatusCode::INTERNAL_SERVER_ERROR, e).into_response())
}

/// Get all todos
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List of todos", body = Vec<TodoItemResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
pub async fn get_all(State(app_state): State<AppState>) -> impl IntoResponse {
    // ... implementation
}

/// Get todo by ID
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i64, Path, description = "Todo item ID")
    ),
    responses(
        (status = 200, description = "Todo found", body = TodoItemResponse),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // ... implementation
}
```

**Why handlers must be outside `impl`?**

The `routes!()` macro from `utoipa-axum` generates additional code that cannot exist inside an `impl` block. This is a limitation of Rust's macro system.

### 3. OpenAPI Configuration

Create `src/config/open_api.rs`:

```rust
use utoipa::OpenApi;
use crate::config::{app_info::AppInfo, settings::AppSettings};

pub const TAG_TODO: &str = "Todo";
pub const TAG_TODO_DESC: &str = "Todo management endpoints";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = TAG_TODO, description = TAG_TODO_DESC)
    )
)]
pub struct ApiDoc;

pub fn configure_openapi(app_info: &AppInfo, app_settings: &AppSettings) -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    let info = app_info.clone();
    doc.info.title = info.name;
    doc.info.version = info.version;
    doc.info.description = Some(info.description);
    doc.servers = Some(vec![utoipa::openapi::ServerBuilder::new()
        .url(format!("http://localhost:{}{}", app_settings.server.port, app_settings.app.prefix))
        .build()]);
    doc
}
```

**Key points:**
- Creates base `ApiDoc` struct with tags
- Overrides info from `Cargo.toml` via `AppInfo`
- Dynamically builds server URL from settings
- Returns configured OpenAPI specification

### 4. App Info Configuration

Create `src/config/app_info.rs`:

```rust
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
```

**Key points:**
- Simple struct with name, version, and description
- Uses `env!()` macro to read from `Cargo.toml` at compile time
- Provides default implementation using package metadata

### 5. Router Setup

Update `src/routes/mod.rs`:

```rust
use axum::{Json, Router, routing::get};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::{app_info, open_api, settings::AppSettings},
    controller::todo_controller::TodoController,
    types::AppState,
};

pub fn create_routes(app_settings: &AppSettings) -> OpenApiRouter<AppState> {
    let app_info = app_info::AppInfo::new();
    let openapi = open_api::configure_openapi(&app_info, app_settings);

    OpenApiRouter::with_openapi(openapi)
        .nest("/todo", TodoController::router())
}

pub fn build_router(app_settings: &AppSettings, app_state: AppState) -> Router {
    let (api_router, api) = create_routes(app_settings).split_for_parts();
    let api_clone = api.clone();
    
    // Build full paths - MUST clone to give ownership to async closure
    let openapi_json_path = format!("{}{}", 
        app_settings.app.prefix, 
        app_settings.openapi.json_path
    );
    let ui_path = app_settings.openapi.ui_path.clone();
    let openapi_json_path_for_route = app_settings.openapi.json_path.clone();

    let api_docs = Router::new()
        .merge(
            SwaggerUi::new(ui_path)
                .url(openapi_json_path, api)
        )
        .route(
            openapi_json_path_for_route.as_str(),
            get(|| async move { Json(api_clone) }),
        )
        .merge(api_router);

    Router::new()
        .nest(&app_settings.app.prefix, api_docs)
        .with_state(app_state)
}
```

**Important note about lifetimes:**

When using closures in async handlers, you must **clone** values to give them ownership:

```rust
// ❌ WRONG - Borrowed reference doesn't live long enough
let path = app_settings.openapi.json_path.as_str();
.route(path, get(|| async move { ... }))

// ✅ CORRECT - Owned String lives as long as needed
let path = app_settings.openapi.json_path.clone();
.route(&path, get(|| async move { ... }))
```

### 6. Update Cargo.toml

Your `Cargo.toml` with project metadata:

```toml
[package]
name = "axum_tutorial"
version = "0.0.1"
edition = "2024"
description = "An axum todo list project"

[dependencies]
async-trait = "0.1.89"
axum = "0.8.7"
config = "0.15.19"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
sqlite = "0.37.0"
tokio = { version = "1.48.0", features = ["full"] }
utoipa = { version = "5.4.0", features = ["axum_extras"] }
utoipa-axum = "0.2.0"
utoipa-swagger-ui = { version = "9.0.2", features = ["axum"] }
```

**Note:** The package metadata (`name`, `version`, `description`) is automatically read by `AppInfo` using `env!()` macros at compile time.

## Configuration

### Application Settings (`src/properties/application.yaml`)

```yaml
server:
  host: "0.0.0.0"
  port: 8080

app:
  prefix: "/api"

openapi:
  ui_path: "/swagger-ui"
  json_path: "/api-docs/openapi.json"
```

**Configuration explanation:**
- `server.host`: Binds to all network interfaces (`0.0.0.0`)
- `server.port`: Server runs on port 8080
- `app.prefix`: All API routes are prefixed with `/api`
- `openapi.ui_path`: Swagger UI accessible at `/swagger-ui`
- `openapi.json_path`: OpenAPI JSON specification at `/api-docs/openapi.json`

## Usage

### Accessing Documentation

After starting the server:

- **Swagger UI**: http://localhost:8080/api/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api/api-docs/openapi.json
- **API Endpoints**: http://localhost:8080/api/todo

### Testing with Swagger UI

1. Navigate to http://localhost:8080/api/swagger-ui
2. Click on any endpoint to expand it
3. Click "Try it out"
4. Fill in the request body/parameters
5. Click "Execute"
6. View the response

### Example: Create Todo via Swagger

1. Go to `POST /api/todo`
2. Click "Try it out"
3. Enter request body:
   ```json
   {
     "title": "Learn Rust",
     "completed": false
   }
   ```
4. Click "Execute"
5. View the 201 response with the created todo

## Troubleshooting

### Issue 1: "Failed to load API definition"

**Symptom:** Swagger UI shows "Not Found /api-docs/openapi.json"

**Solution:** Ensure the OpenAPI JSON path in SwaggerUI matches the actual route:

```rust
// URL must match the route endpoint
SwaggerUi::new("/swagger-ui")
    .url("/api/api-docs/openapi.json", api)  // Full path from root

// And the route must serve it
.route("/api-docs/openapi.json", get(handler))
```

### Issue 2: "Overlapping method route"

**Symptom:** `Handler for GET /{id} already exists`

**Solution:** Ensure each handler is registered only once:

```rust
// ✅ CORRECT
OpenApiRouter::new()
    .routes(routes!(
        create_todo,
        get_all,
        get_by_id,
        update_todo,
        delete_by_id
    ))

// ❌ WRONG - Duplicate registration
OpenApiRouter::new()
    .routes(routes!(get_all))
    .routes(routes!(get_by_id))  // Both are GET, conflict!
```

### Issue 3: "Borrowed data escapes outside of function"

**Symptom:** Lifetime error when using references in async closures

**Solution:** Clone strings to give ownership:

```rust
// ❌ WRONG
let path = settings.json_path.as_str();  // Borrowed
.route(path, get(|| async move { ... }))

// ✅ CORRECT
let path = settings.json_path.clone();  // Owned
.route(&path, get(|| async move { ... }))
```

### Issue 4: "struct is not supported in `trait`s or `impl`s"

**Symptom:** Error when using `#[utoipa::path]` inside `impl` block

**Solution:** Move handlers outside the `impl` block:

```rust
pub struct TodoController;

impl TodoController {
    pub fn router() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().routes(routes!(create_todo))
    }
}

// ✅ Handler outside impl
#[utoipa::path(...)]
pub async fn create_todo(...) -> impl IntoResponse {
    // ...
}
```

## Best Practices

### 1. Keep Documentation Close to Code

```rust
#[utoipa::path(
    post,
    path = "/",
    request_body = TodoItemRequest,
    responses(
        (status = 201, description = "Todo created successfully", body = TodoItemResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
pub async fn create_todo(...) {
    // Documentation is right above the implementation
}
```

### 2. Use Schema Examples

```rust
#[derive(ToSchema)]
pub struct TodoItemRequest {
    #[schema(example = "Buy groceries")]
    pub title: String,
    
    #[schema(example = false)]
    pub completed: bool,
}
```

### 3. Document All Response Codes

```rust
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "Todo found", body = TodoItemResponse),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
```

### 4. Use Descriptive Tags

```rust
#[utoipa::path(
    // ...
    tag = "todos"  // Groups related endpoints in Swagger UI
)]
```

## References

- [utoipa Documentation](https://docs.rs/utoipa/)
- [utoipa-axum Documentation](https://docs.rs/utoipa-axum/)
- [OpenAPI Specification](https://spec.openapis.org/oas/latest.html)
- [Swagger UI](https://swagger.io/tools/swagger-ui/)

---

**Built with ❤️ using Rust and OpenAPI**
