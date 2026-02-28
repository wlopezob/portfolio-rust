use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request model for creating or updating a todo item
/// 
/// This structure represents the data required to create or update a todo item
#[derive(Serialize, Deserialize, Default, Clone, Debug, 
    utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TodoItemRequest {
    /// Optional identifier (used for updates, omit for creation)
    #[schema(example = 1)]
    pub id: Option<i64>,
    
    /// Title or description of the todo task
    #[schema(example = "Buy groceries")]
    #[validate(length(min = 5, message = "Title cannot be empty and must be at least 5 characters long"))]
    pub title: String,
    
    /// Indicates whether the todo item is completed
    #[schema(example = false)]
    pub completed: bool,

    /// Deadline for the todo item in DD/MM/YYYY format
    #[schema(example = "31/12/2025")]
    #[validate(custom(function = "validate_date_ddmmyyyy"))]
    pub max_date: String,
}

fn make_error(code: &'static str, message: &'static str) -> validator::ValidationError {
    let mut err = validator::ValidationError::new(code);
    err.message = Some(std::borrow::Cow::Borrowed(message));
    err
}

fn validate_date_ddmmyyyy(date: &str) -> Result<(), validator::ValidationError> {
    let parts: Vec<&str> = date.split('/').collect();
    if parts.len() != 3 || parts[0].len() != 2 || parts[1].len() != 2 || parts[2].len() != 4 {
        return Err(make_error("invalid_date_format", "Date must be in DD/MM/YYYY format"));
    }
    let day: u32 = parts[0].parse().map_err(|_| make_error("invalid_date_format", "Date must be in DD/MM/YYYY format"))?;
    let month: u32 = parts[1].parse().map_err(|_| make_error("invalid_date_format", "Date must be in DD/MM/YYYY format"))?;
    let year: i32 = parts[2].parse().map_err(|_| make_error("invalid_date_format", "Date must be in DD/MM/YYYY format"))?;

    let input_date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| make_error("invalid_date_value", "Date must be in DD/MM/YYYY format with valid values"))?;

    let today = Local::now().date_naive();
    if input_date < today {
        return Err(make_error("date_must_be_present_or_future", "Date must be greater than or equal to current date"));
    }

    Ok(())
}