use serde::Serialize;
use sqlx::Error as SqlxError;
use std::{borrow::Cow, collections::HashMap};
use thiserror::Error;
use validator::ValidationErrors;

/// Structure for the JSON error response.
///
/// This struct contains:
/// - **error**: the main error message.
/// - **details**: optional field-specific validation errors.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// The main error message.
    pub error: String,

    /// Optional validation error details.
    ///
    /// Each key corresponds to a field name, and the value is a list of
    /// error messages for that field.
    pub details: Option<HashMap<String, Vec<String>>>,
}

/// Enum to encapsulate service errors
#[derive(Debug, Error)]
pub enum ServiceError {
    // Handle validation errors
    #[error("Validation failed")]
    ValidationError(#[from] ValidationErrors),

    // Handle SQLx database errors
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),
}

impl ServiceError {
    /// Convert an error into `ErrorResponse` to send it as JSON
    pub fn to_error_response(&self) -> ErrorResponse {
        match self {
            // In case of validation error, extract detailed messages
            ServiceError::ValidationError(validation_errors) => {
                let mut error_details = HashMap::new();

                for (field, errors) in validation_errors.field_errors().iter() {
                    let messages: Vec<String> = errors
                        .iter()
                        .map(|error| {
                            error
                                .message
                                .clone()
                                .unwrap_or_else(|| {
                                    Cow::Borrowed("Invalid value")
                                })
                                .into_owned()
                        })
                        .collect();

                    error_details.insert(field.to_string(), messages);
                }

                ErrorResponse {
                    error: "Validation failed".to_string(),
                    details: Some(error_details),
                }
            }

            // In case of database error
            ServiceError::DatabaseError(_) => ErrorResponse {
                error: "A database error occurred".to_string(),
                details: None,
            },
        }
    }
}
