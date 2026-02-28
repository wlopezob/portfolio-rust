# Validation

This document describes the validation architecture used in this project.

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `validator` | Derive-based struct validation via `#[validate(...)]` annotations |
| `chrono` | Current date resolution for deadline validation |

---

## Architecture

Validation runs in two steps inside the custom `ValidatedJson<T>` extractor, before the request reaches the controller handler:

```
HTTP Request
    │
    ▼
ValidatedJson<T>          (src/validators/validated_json.rs)
    ├── 1. JSON deserialization  →  400 Bad Request on parse error
    └── 2. .validate()          →  400 Bad Request on rule violation
    │
    ▼
Handler (controller)
```

All errors are returned as `ApiException::BadRequest`, which produces a consistent JSON response:

```json
{
  "code": "400 Bad Request",
  "message": "<validation error message>"
}
```

---

## Custom Extractor — `ValidatedJson<T>`

**File:** `src/validators/validated_json.rs`

A generic axum extractor that implements `FromRequest`. It combines JSON deserialization and struct validation into a single step, mapping both failure types to `ApiException`.

```rust
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiException;
    ...
}
```

**Usage in controllers:**

```rust
pub async fn create_todo(
    State(app_state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<TodoItemRequest>,
) -> impl IntoResponse { ... }
```

Applied to: `POST /api/todo` and `PUT /api/todo/{id}`.

---

## Request Validation — `TodoItemRequest`

**File:** `src/model/todo_item_request.rs`

### Fields and rules

| Field | Type | Rule | Error message |
|-------|------|------|---------------|
| `title` | `String` | Minimum length of 5 characters | `Title cannot be empty and must be at least 5 characters long` |
| `maxDate` | `String` | Custom: `validate_date_ddmmyyyy` | See below |

> Note: `#[serde(rename_all = "camelCase")]` is applied to the struct, so `max_date` is received as `maxDate` in JSON requests.

---

## Custom Validator — `validate_date_ddmmyyyy`

**File:** `src/model/todo_item_request.rs`

A pure Rust function registered via `#[validate(custom(function = "validate_date_ddmmyyyy"))]`. It runs three sequential checks:

### 1. Format check — `DD/MM/YYYY`

Ensures the value has exactly 3 parts separated by `/`, with lengths `2/2/4`.

```
"1/1/2025"     → invalid  (parts don't have correct length)
"01-01-2025"   → invalid  (wrong separator)
"01/01/2025"   → proceeds to next check
```

**Error code:** `invalid_date_format`
**Message:** `Date must be in DD/MM/YYYY format`

### 2. Calendar validity check

Uses `chrono::NaiveDate::from_ymd_opt` to reject dates that don't exist on the calendar.

```
"31/02/2025"   → invalid  (February has no 31st)
"00/01/2025"   → invalid  (day 0 does not exist)
"01/13/2025"   → invalid  (month 13 does not exist)
"28/02/2025"   → proceeds to next check
```

**Error code:** `invalid_date_value`
**Message:** `Date must be in DD/MM/YYYY format with valid values`

### 3. Present or future check

Compares the parsed date against today's local date. Past dates are rejected.

```
"01/01/2020"   → invalid  (past date)
"<today>"      → valid
"31/12/2099"   → valid
```

**Error code:** `date_must_be_present_or_future`
**Message:** `Date must be greater than or equal to current date`

---

## Date Storage — `to_iso_date` / `to_display_date`

**File:** `src/utils/utils.rs`

Although the API accepts and returns dates in `DD/MM/YYYY` format, the database stores them as `YYYY-MM-DD` (ISO 8601) so SQLite date functions work correctly.

Conversion happens transparently in the repository layer:

| Direction | Function | Example |
|-----------|----------|---------|
| Request → DB (write) | `to_iso_date` | `"27/02/2026"` → `"2026-02-27"` |
| DB → Response (read) | `to_display_date` | `"2026-02-27"` → `"27/02/2026"` |

---

## Error response examples

**Invalid format:**
```json
{
  "code": "400 Bad Request",
  "message": "max_date: Date must be in DD/MM/YYYY format"
}
```

**Invalid calendar date:**
```json
{
  "code": "400 Bad Request",
  "message": "max_date: Date must be in DD/MM/YYYY format with valid values"
}
```

**Past date:**
```json
{
  "code": "400 Bad Request",
  "message": "max_date: Date must be greater than or equal to current date"
}
```

**Title too short:**
```json
{
  "code": "400 Bad Request",
  "message": "title: Title cannot be empty and must be at least 5 characters long"
}
```
