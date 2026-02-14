# TodoList SQLite - Rust REST API

A production-ready REST API built with Rust, featuring a clean architecture pattern with Controllers and Services layers, similar to modern backend frameworks.

## 🚀 Features

- **Clean Architecture**: Separation of concerns with Controller → Service layers
- **Dependency Injection**: Using traits (interfaces) for loose coupling
- **Async/Await**: Full async support with Tokio runtime
- **Type Safety**: Leveraging Rust's strong type system
- **RESTful API**: Complete CRUD operations
- **SQLite Database**: Lightweight embedded database
- **OpenAPI/Swagger**: Auto-generated interactive API documentation
- **Configuration Management**: External configuration file support
- **Hot Reload**: Development mode with auto-restart

## 📋 Table of Contents

- [Architecture](#️-architecture)
- [Tech Stack](#️-tech-stack)
- [Installation](#-installation)
- [Usage](#-usage)
- [API Endpoints](#-api-endpoints)
- [OpenAPI & Swagger UI](#-openapi--swagger-ui)
- [Error Handling with thiserror](#-error-handling-with-thiserror)
- [Configuration Manager](#️-configuration-manager)
- [Project Structure](#-project-structure)
- [Sequence Diagrams](#-sequence-diagrams)
- [Development](#-development)
- [Design Patterns Used](#-design-patterns-used)
- [Best Practices](#-best-practices)

## 🏗️ Architecture

This project follows a layered architecture pattern:

```
┌─────────────────┐
│   Controller    │  HTTP handling, routing
├─────────────────┤
│    Service      │  Business logic & data access
├─────────────────┤
│    Database     │  SQLite
└─────────────────┘
```

### Key Components

- **Controllers**: Handle HTTP requests and responses
- **Services**: Contain business logic and database operations
- **Models**: Request/Response DTOs
- **Types**: Shared types and application state

## 🛠️ Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Ergonomic web framework
- **Runtime**: [Tokio](https://tokio.rs/) - Async runtime
- **Database**: [SQLite](https://www.sqlite.org/) via `sqlite` crate
- **Serialization**: [Serde](https://serde.rs/) - JSON handling
- **Documentation**: [utoipa](https://github.com/juhaku/utoipa) - OpenAPI generation
- **API Testing**: [utoipa-swagger-ui](https://github.com/juhaku/utoipa) - Interactive UI
- **Configuration**: Custom config management

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Setup

```bash
# Clone the repository
git clone https://github.com/wlopezob/portfolio-rust.git
cd todolist-sqlite

# Build the project
cargo build

# Run the application
cargo run
```

## 🚀 Usage

The server will start on `http://127.0.0.1:8080` by default.

### Environment Configuration

Create or edit `src/properties/application.yaml`:

```yaml
app:
  prefix: /api/v1

server:
  host: 127.0.0.1
  port: 8080
```

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/todo` | Create a new todo |
| `GET` | `/api/v1/todo` | Get all todos |
| `GET` | `/api/v1/todo/{id}` | Get todo by ID |
| `PUT` | `/api/v1/todo/{id}` | Update todo |
| `DELETE` | `/api/v1/todo/{id}` | Delete todo |

### Request Examples

**Create Todo**
```bash
curl -X POST http://localhost:8080/api/v1/todo \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "completed": false}'
```

**Get All Todos**
```bash
curl http://localhost:8080/api/v1/todo
```

**Update Todo**
```bash
curl -X PUT http://localhost:8080/api/v1/todo/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "completed": true}'
```

**Delete Todo**
```bash
curl -X DELETE http://localhost:8080/api/v1/todo/1
```

## 📚 OpenAPI & Swagger UI

This project includes **automatic API documentation** using OpenAPI 3.0 and an interactive Swagger UI interface.

### Access Documentation

- **Swagger UI**: http://localhost:8080/api/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api/api-docs/openapi.json

### Features

✅ **Auto-generated documentation** from Rust code  
✅ **Interactive testing** directly in the browser  
✅ **Type-safe** - Documentation synchronized with code  
✅ **Zero runtime overhead** - Generated at compile time  

### Quick Start

1. Start the server: `cargo run`
2. Open http://localhost:8080/api/swagger-ui in your browser
3. Try out endpoints directly from the UI:
   - Click on any endpoint to expand
   - Click "Try it out"
   - Fill in parameters/request body
   - Click "Execute" to see the response

### Implementation

The OpenAPI integration uses:
- **`utoipa`** - Core OpenAPI generation
- **`utoipa-axum`** - Axum-specific helpers
- **`utoipa-swagger-ui`** - Embedded Swagger UI

For detailed implementation guide, see **[OPENAPI.md](OPENAPI.md)**

## 🔧 Error Handling with thiserror

This project implements **robust error handling** using `thiserror` for automatic error type conversions and comprehensive error propagation throughout all application layers.

### Features

✅ **Automatic error conversion** with `From` trait implementation  
✅ **Type-safe error propagation** using the `?` operator  
✅ **Custom error types** with meaningful messages  
✅ **HTTP status code mapping** for REST API responses  

### Quick Example

**Repository Layer (Automatic Conversion):**
```rust
async fn get_by_id(&self, id: i64) -> Result<Option<TodoItemModel>, ApiException> {
    let connection = self.db.lock()?;  // PoisonError → ApiException
    let mut statement = connection.prepare(query)?;  // sqlite::Error → ApiException
    statement.bind((1, id))?;  // Automatic conversion
    
    if statement.next()? == sqlite::State::Row {
        Ok(Some(TodoItemModel {
            id: statement.read::<i64, _>("id")?,  // Clean with ?
            title: statement.read::<String, _>("title")?,
            completed: statement.read::<i64, _>("completed")? != 0,
        }))
    } else {
        Ok(None)
    }
}
```

**Service Layer (Business Logic):**
```rust
async fn get_by_id(&self, id: i64) -> Result<TodoItemResponse, ApiException> {
    self.todo_repository.get_by_id(id).await?
        .ok_or_else(|| ApiException::NotFound(format!("Todo item with {} not found", id)))
        .map(|row| TodoItemResponse { /* transform */ })
}
```

### Error Response Format

All errors return a standardized JSON response:

```json
{
  "code": "404 Not Found",
  "message": "Todo item with 5 not found"
}
```

### Key Benefits

1. **No repetitive error handling**: The `?` operator automatically converts errors
2. **Type safety**: Compiler ensures all errors are handled
3. **Consistent responses**: All errors follow the same JSON format
4. **Easy debugging**: Descriptive error messages with context
5. **Clean code**: No `.unwrap()` or `.expect()` in production code

### Implementation Details

For comprehensive documentation including:
- Complete error architecture
- Layer-by-layer implementation
- Automatic conversion patterns
- Best practices and troubleshooting
- Common patterns and examples

See **[THISERROR.md](THISERROR.md)**

## ⚙️ Configuration Manager

This project uses a **multi-environment configuration system** with YAML files, environment-based profiles, and automatic `.env` file loading for managing application settings.

### Features

✅ **Profile-based configuration** - Separate settings for dev, staging, and production  
✅ **YAML configuration files** - Easy to read and maintain  
✅ **Type-safe configuration** - Rust structs ensure validity  
✅ **Configuration merging** - Base settings with profile overrides  
✅ **Environment variables** - Via `PROFILE` from `.env` files  
✅ **Automatic loading** - Using `dotenvy` crate  

### Quick Start

**1. Setup environment:**
```bash
# Copy template
cp .env.example .env

# Edit profile (already set to dev by default)
echo "PROFILE=dev" > .env
```

**2. Run application:**
```bash
cargo run
# Automatically loads .env and uses application-dev.yaml
```

**3. Switch environments:**
```bash
# For production
echo "PROFILE=prod" > .env
cargo run
```

### Configuration Flow

```
.env file (PROFILE=dev)
    ↓
dotenvy loads into environment
    ↓
config reads PROFILE variable
    ↓
Loads application.yaml + application-dev.yaml
    ↓
Type-safe AppSettings struct
```

### Implementation

The configuration system uses:
- **`config`** - Hierarchical YAML configuration
- **`dotenvy`** - Automatic `.env` file loading
- **`serde`** - Type-safe deserialization

For detailed implementation guide, see **[CONF-MANAGER.md](CONF-MANAGER.md)**

## 📁 Project Structure

```
todolist-sqlite/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/              # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── controller/          # HTTP request handlers
│   │   ├── mod.rs
│   │   └── todo_controller.rs
│   ├── service/             # Business logic & data access
│   │   ├── mod.rs
│   │   └── todo_service.rs
│   ├── model/               # DTOs
│   │   ├── mod.rs
│   │   ├── todo_item_request.rs
│   │   └── todo_item_response.rs
│   ├── routes/              # Route configuration
│   │   └── mod.rs
│   ├── types/               # Shared types
│   │   └── mod.rs
│   └── properties/          # Config files
│       └── application.yaml
├── data/
│   └── todo.db              # SQLite database
├── Cargo.toml
└── README.md
```

## 📊 Sequence Diagrams

### Create Todo Flow

```mermaid
sequenceDiagram
    participant Client
    participant Controller
    participant Service
    participant Database

    Client->>Controller: POST /api/v1/todo
    activate Controller
    
    Controller->>Controller: Parse JSON to TodoItemRequest
    Controller->>Service: create(db, TodoItemRequest)
    activate Service
    
    Service->>Service: Validate business rules
    Service->>Database: INSERT INTO todos
    activate Database
    Database-->>Service: Return inserted ID
    deactivate Database
    
    Service->>Database: SELECT last_insert_rowid()
    activate Database
    Database-->>Service: ID
    deactivate Database
    
    Service->>Service: Build TodoItemResponse
    Service-->>Controller: TodoItemResponse
    deactivate Service
    
    Controller-->>Client: 201 Created + JSON
    deactivate Controller
```

### Get All Todos Flow

```mermaid
sequenceDiagram
    participant Client
    participant Controller
    participant Service
    participant Database

    Client->>Controller: GET /api/v1/todo
    activate Controller
    
    Controller->>Service: find_all(db)
    activate Service
    
    Service->>Database: SELECT * FROM todos
    activate Database
    Database-->>Service: Vec<Row>
    deactivate Database
    
    Service->>Service: Map rows to TodoItemResponse
    Service-->>Controller: Vec<TodoItemResponse>
    deactivate Service
    
    Controller-->>Client: 200 OK + JSON Array
    deactivate Controller
```

### Update Todo Flow

```mermaid
sequenceDiagram
    participant Client
    participant Controller
    participant Service
    participant Database

    Client->>Controller: PUT /api/v1/todo/{id}
    activate Controller
    
    Controller->>Controller: Extract id & payload
    Controller->>Service: update(db, id, TodoItemRequest)
    activate Service
    
    Service->>Service: Validate business rules
    Service->>Database: SELECT WHERE id = ?
    activate Database
    Database-->>Service: Check if exists
    
    alt Todo exists
        Service->>Database: UPDATE todos SET ... WHERE id = ?
        Database-->>Service: Success
        Service-->>Controller: TodoItemResponse
        Controller-->>Client: 200 OK + JSON
    else Todo not found
        Database-->>Service: Not found
        deactivate Database
        Service-->>Controller: Error("Not found")
        Controller-->>Client: 404 Not Found
    end
    
    deactivate Service
    deactivate Controller
```

### Delete Todo Flow

```mermaid
sequenceDiagram
    participant Client
    participant Controller
    participant Service
    participant Database

    Client->>Controller: DELETE /api/v1/todo/{id}
    activate Controller
    
    Controller->>Service: delete(db, id)
    activate Service
    
    Service->>Database: SELECT WHERE id = ?
    activate Database
    Database-->>Service: Check if exists
    
    alt Todo exists
        Service->>Database: DELETE FROM todos WHERE id = ?
        Database-->>Service: Success
        Service-->>Controller: Ok(())
        Controller-->>Client: 204 No Content
    else Todo not found
        Database-->>Service: Not found
        deactivate Database
        Service-->>Controller: Error("Not found")
        Controller-->>Client: 400 Bad Request
    end
    
    deactivate Service
    deactivate Controller
```

## 🔧 Development

### Configuration

Edit `src/properties/application.yaml`:

```yaml
app:
  prefix: /api/v1

server:
  host: 127.0.0.1
  port: 8080
```

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);
```

## 🎯 Design Patterns Used

- **Dependency Injection**: Via `AppState` and trait objects
- **Service Layer Pattern**: Business logic encapsulation
- **DTO Pattern**: Request/Response models separate from domain
- **Trait-based Polymorphism**: `TodoService` trait for testability
- **Result Pattern**: Error handling with `Result<T, E>`
- **Builder Pattern**: Axum's Router builder

## 🔐 Best Practices

✅ **Type Safety**: Strong typing throughout  
✅ **Error Handling**: Proper Result types and error propagation  
✅ **Separation of Concerns**: Layered architecture  
✅ **Async/Await**: Non-blocking I/O operations  
✅ **Configuration**: External configuration files  
✅ **Dependency Injection**: Loose coupling via traits  

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📧 Contact

For questions or suggestions, please open an issue on GitHub.

---

**Built with ❤️ using Rust**