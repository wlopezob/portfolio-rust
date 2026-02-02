# Error Handling with thiserror - Implementation Guide

This document explains how robust error handling is implemented in this Rust/Axum project using `thiserror` for automatic error type conversions and comprehensive error propagation.

## 📋 Table of Contents

- [Implementation](#implementation)
- [Error Flow by Layer](#error-flow-by-layer)
- [Overview](#overview)
- [Why thiserror?](#why-thiserror)
- [Error Architecture](#error-architecture)
- [Automatic Conversions](#automatic-conversions)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Troubleshooting](#troubleshooting)

## Implementation

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
thiserror = "2.0.18"
```

### Key Components

1. **`#[derive(Error)]`** - Implements `std::error::Error` trait
2. **`#[error("...")]`** - Defines the error message format
3. **`#[from]`** - Generates automatic `From` implementation
4. **`impl IntoResponse`** - Converts errors to HTTP responses

## Error Flow by Layer

### Layer 1: Repository (Data Access)

**File**: `src/repository/todo_repository.rs`

```rust
use crate::model::api_exception::ApiException;

#[async_trait]
pub trait TodoRepositoryInterface: Send + Sync {
    async fn create(&self, todo_item: TodoItemModel) -> Result<TodoItemModel, ApiException>;
    async fn get_all(&self) -> Result<Vec<TodoItemModel>, ApiException>;
    async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException>;
}

#[async_trait]
impl TodoRepositoryInterface for TodoRepositoryImpl {
    async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;  // ✅ PoisonError → ApiException
        let query = "SELECT id, title, completed FROM todos WHERE id = ?";

        let mut statement = connection.prepare(query)?;  // ✅ sqlite::Error → ApiException
        statement.bind((1, id))?;  // ✅ sqlite::Error → ApiException

        if statement.next()? == sqlite::State::Row {  // ✅ sqlite::Error → ApiException
            Ok(Some(TodoItemModel {
                id: statement.read::<i64, _>("id")?,  // ✅ sqlite::Error → ApiException
                title: statement.read::<String, _>("title")?,
                completed: statement.read::<i64, _>("completed")? != 0,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_all(&self) -> Result<Vec<TodoItemModel>, ApiException> {
        let connection = self.db.lock()?;
        let query = "SELECT id, title, completed FROM todos";

        let mut statement = connection.prepare(query)?;

        let todos: Result<Vec<TodoItemModel>, sqlite::Error> = statement
            .iter()
            .map(|row| {
                let row = row?;
                Ok(TodoItemModel {
                    id: row.read::<i64, _>("id"),
                    title: row.read::<&str, _>("title").to_string(),
                    completed: row.read::<i64, _>("completed") != 0,
                })
            })
            .collect();

        Ok(todos?)  // ✅ sqlite::Error → ApiException
    }
}
```

**Key Points:**
- All SQLite errors automatically convert to `ApiException::DatabaseError`
- Mutex lock errors automatically convert to `ApiException::DatabaseLockError`
- Repository returns `Option` for "not found" scenarios (not an error at this level)

### Layer 2: Service (Business Logic)

**File**: `src/service/todo_service.rs`

```rust
use crate::model::api_exception::ApiException;

#[async_trait::async_trait]
pub trait TodoServiceInterface: Send + Sync {
    async fn create(&self, todo_item: TodoItemRequest) -> Result<TodoItemResponse, ApiException>;
    async fn get_all(&self) -> Result<Vec<TodoItemResponse>, ApiException>;
    async fn get_by_id(&self, id: i64) -> Result<TodoItemResponse, ApiException>;
}

#[async_trait::async_trait]
impl TodoServiceInterface for TodoServiceImpl {
    async fn get_by_id(&self, id: i64) -> Result<TodoItemResponse, ApiException> {
        // Functional approach with error handling
        self.todo_repository.get_by_id(id).await?
            .ok_or_else(|| ApiException::NotFound(format!("Todo item with {} not found", id)))
            .map(|row| TodoItemResponse {
                id: Some(row.id),
                title: row.title,
                completed: row.completed,
            })
    }

    async fn get_all(&self) -> Result<Vec<TodoItemResponse>, ApiException> {
        let item_responses = self.todo_repository.get_all().await?;  // ✅ Propagates errors
        
        let todos = item_responses
            .iter()
            .map(|row| TodoItemResponse {
                id: Some(row.id),
                title: row.title.clone(),
                completed: row.completed,
            })
            .collect::<Vec<TodoItemResponse>>();
        
        Ok(todos)
    }
}
```

**Key Points:**
- Service layer adds business logic errors (like `NotFound`)
- Uses `.ok_or_else()` to convert `Option<T>` to `Result<T, E>`
- Errors from repository layer propagate automatically with `?`

### Layer 3: Controller (HTTP Handling)

**File**: `src/controller/todo_controller.rs`

```rust
use crate::model::api_exception::ApiException;

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    app_state
        .todo_service
        .get_by_id(id)
        .await
        .map(|todo| (StatusCode::OK, Json(todo)).into_response())
    // ✅ ApiException automatically converts to HTTP response
}

pub async fn get_all(
    State(app_state): State<AppState>
) -> impl IntoResponse {
    app_state
        .todo_service
        .get_all()
        .await
        .map(|todos| (StatusCode::OK, Json(todos)).into_response())
    // ✅ ApiException automatically converts to HTTP response
}
```

**Key Points:**
- No explicit error handling needed (Axum handles it)
- `ApiException` implements `IntoResponse` for automatic HTTP conversion
- Clean, functional style with `.map()`

## Overview

This project uses `thiserror` to create a centralized error handling system that:

- ✅ Automatically converts low-level errors (SQLite, Mutex) to application errors
- ✅ Propagates errors cleanly using the `?` operator
- ✅ Maps errors to appropriate HTTP status codes
- ✅ Returns consistent JSON error responses

## Why thiserror?

### Without thiserror (Manual Error Handling)

```rust
// ❌ Repetitive and error-prone
async fn get_by_id(&self, id: i64) -> Result<TodoItemModel, ApiException> {
    let connection = self.db.lock()
        .map_err(|e| ApiException::DatabaseLockError(e.to_string()))?;
    
    let mut statement = connection.prepare(query)
        .map_err(|e| ApiException::DatabaseError(e.to_string()))?;
    
    statement.bind((1, id))
        .map_err(|e| ApiException::DatabaseError(e.to_string()))?;
    
    // ... more repetitive map_err calls
}
```

### With thiserror (Automatic Conversion)

```rust
// ✅ Clean and concise
async fn get_by_id(&self, id: i64) -> Result<TodoItemModel, ApiException> {
    let connection = self.db.lock()?;  // Automatic conversion
    let mut statement = connection.prepare(query)?;  // Automatic conversion
    statement.bind((1, id))?;  // Automatic conversion
    
    // ... clean code with ? operator
}
```

## Error Architecture

### Central Error Type

Located in `src/model/api_exception.rs`:

```rust
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use std::sync::PoisonError;

#[derive(Error, Debug)]
pub enum ApiException {
    #[error("{0}")]
    InternalError(String),
    
    #[error("Database lock error: {0}")]
    DatabaseLockError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),  // 🎯 Automatic conversion

    #[error("{0}")]
    NotFound(String),
}

impl ApiException {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiException::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::DatabaseLockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        ApiException::InternalError(msg.into())
    }
}

// Manual conversion for PoisonError (generic type)
impl<T> From<PoisonError<T>> for ApiException {
    fn from(err: PoisonError<T>) -> Self {
        ApiException::DatabaseLockError(err.to_string())
    }
}

#[derive(Serialize, ToSchema)]
pub struct ApiExceptionResponseMessage {
    pub code: String,
    pub message: String,
}

impl IntoResponse for ApiException {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        (
            status_code,
            Json(ApiExceptionResponseMessage {
                code: status_code.as_str().to_string(),
                message: self.to_string(),
            }),
        ).into_response()
    }
}
```

## Automatic Conversions

### With `#[from]` attribute (preferred for simple types)

```rust
#[error("Database error: {0}")]
DatabaseError(#[from] sqlite::Error),
```

**Generated code:**
```rust
impl From<sqlite::Error> for ApiException {
    fn from(err: sqlite::Error) -> Self {
        ApiException::DatabaseError(err)
    }
}
```

### With manual `impl From` (for generic types)

```rust
impl<T> From<PoisonError<T>> for ApiException {
    fn from(err: PoisonError<T>) -> Self {
        ApiException::DatabaseLockError(err.to_string())
    }
}
```

**Why manual?** 
- `PoisonError<T>` is generic
- We want to convert it to a `String` (loses type information but keeps message)

## Best Practices

### ✅ DO

1. **Use `?` for error propagation**
   ```rust
   let connection = self.db.lock()?;  // Clean and simple
   ```

2. **Define error variants for business logic**
   ```rust
   ApiException::NotFound(format!("Todo with id {} not found", id))
   ```

3. **Use `.ok_or_else()` to convert `Option` to `Result`**
   ```rust
   self.repository.get_by_id(id).await?
       .ok_or_else(|| ApiException::NotFound("Not found".into()))
   ```

4. **Keep error messages descriptive**
   ```rust
   #[error("Database lock error: {0}")]
   ```

### ❌ DON'T

1. **Don't use `.unwrap()` or `.expect()` in production code**
   ```rust
   let connection = self.db.lock().unwrap();  // ❌ Can panic!
   ```

2. **Don't use `.ok()` when you need error propagation**
   ```rust
   let id = statement.read::<i64, _>(0).ok()?;  // ❌ Loses error information
   let id = statement.read::<i64, _>(0)?;  // ✅ Propagates error
   ```

3. **Don't use `.unwrap_or_default()` for database operations**
   ```rust
   let id = statement.read::<i64, _>(0).unwrap_or_default();  // ❌ Hides errors!
   ```

4. **Don't catch and ignore errors**
   ```rust
   if let Err(_) = operation() {
       // ❌ Ignoring errors
   }
   ```

## Common Patterns

### Pattern 1: Repository - Database Operations

```rust
async fn operation(&self) -> Result<T, ApiException> {
    let connection = self.db.lock()?;
    let mut statement = connection.prepare("...")?;
    statement.bind((1, value))?;
    statement.next()?;
    Ok(result)
}
```

### Pattern 2: Service - Option to Result

```rust
async fn get_by_id(&self, id: i64) -> Result<T, ApiException> {
    self.repository.get_by_id(id).await?
        .ok_or_else(|| ApiException::NotFound(format!("Not found: {}", id)))
        .map(|item| /* transform */)
}
```

### Pattern 3: Controller - Automatic Error Handling

```rust
pub async fn handler(...) -> impl IntoResponse {
    app_state
        .service
        .method()
        .await
        .map(|result| (StatusCode::OK, Json(result)).into_response())
}
```

### Pattern 4: Iterator with Results

```rust
let items: Result<Vec<T>, sqlite::Error> = iterator
    .map(|item| {
        let item = item?;
        Ok(transform(item))
    })
    .collect();

Ok(items?)  // Converts sqlite::Error → ApiException
```

## Troubleshooting

### Issue 1: "cannot use `?` operator"

**Error:**
```
the `?` operator can only be used in a function that returns `Result`
```

**Solution:** Change function signature to return `Result`:
```rust
// ❌ Before
async fn handler() -> Response {

// ✅ After
async fn handler() -> Result<impl IntoResponse, ApiException> {
```

### Issue 2: "the trait `From<ErrorType>` is not implemented"

**Error:**
```
the trait `From<sqlite::Error>` is not implemented for `ApiException`
```

**Solution:** Add `#[from]` or implement `From` manually:
```rust
#[error("Database error: {0}")]
DatabaseError(#[from] sqlite::Error),
```

### Issue 3: "mismatched types, expected Result found Option"

**Error:**
```
expected enum `Result<_, ApiException>`
found enum `Option<_>`
```

**Solution:** Use `.ok_or_else()` to convert:
```rust
repository.get_by_id(id).await?
    .ok_or_else(|| ApiException::NotFound("Not found".into()))
```

### Issue 4: "borrowed data escapes outside of method"

**Error:**
```
`self` escapes the method body here
lifetime `'life0` must outlive `'static`
```

**Solution:** Convert error to String instead of keeping reference:
```rust
// ❌ Wrong
DatabaseError(#[from] PoisonError<MutexGuard<'static, Connection>>),

// ✅ Correct
impl<T> From<PoisonError<T>> for ApiException {
    fn from(err: PoisonError<T>) -> Self {
        ApiException::DatabaseLockError(err.to_string())
    }
}
```

## HTTP Response Examples

### Success Response

```json
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 1,
  "title": "Learn Rust",
  "completed": false
}
```

### Error Response - Not Found

```json
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "code": "404 Not Found",
  "message": "Todo item with 5 not found"
}
```

### Error Response - Database Error

```json
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "code": "500 Internal Server Error",
  "message": "Database error: no such table: todos"
}
```

### Error Response - Lock Error

```json
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "code": "500 Internal Server Error",
  "message": "Database lock error: poisoned lock: another task failed inside"
}
```

## Summary

### Benefits of this Approach

1. ✅ **No Repetitive Code**: Use `?` everywhere, automatic conversion
2. ✅ **Type Safety**: Compiler ensures all errors are handled
3. ✅ **Consistent Responses**: All errors follow the same JSON format
4. ✅ **Easy Debugging**: Descriptive messages with context
5. ✅ **Clean Codebase**: No `.unwrap()`, `.expect()`, or verbose `map_err`
6. ✅ **Production Ready**: Proper error handling without panics

### Error Handling Flow

```
Low-level Error (sqlite::Error, PoisonError)
    ↓
Automatic conversion (via #[from] or impl From)
    ↓
ApiException (centralized error type)
    ↓
IntoResponse implementation
    ↓
HTTP Response (with appropriate status code + JSON)
    ↓
Client receives structured error
```

---

**Built with ❤️ using Rust and thiserror**
