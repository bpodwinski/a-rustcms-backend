use anyhow::Error as AnyhowError;
use log::{error, info, warn};
use ntex::http::StatusCode;
use ntex::web::error::InternalError;
use ntex::web::{self, HttpResponse};
use serde_json::error::Error as SerdeJsonError;
use sqlx::Error as SqlxError; // Importer le type d'erreur SQLx
use validator::ValidationErrors;

use crate::middlewares::error_middleware::Error;

/// Maps `anyhow::Error` to `ntex::web::Error`, handling specific error types and logging them.
pub fn convert_anyhow_to_ntex(e: AnyhowError) -> web::Error {
    let error_message: String;
    let mut backtrace = None;

    // Determine the type of error and assign an appropriate HTTP status code
    let status_code =
        if let Some(validation_errors) = e.downcast_ref::<ValidationErrors>() {
            // Handle validation errors
            error_message = format!(
                "Validation error: {}",
                format_validation_errors(validation_errors)
            );
            warn!("Validation error occurred: {:?}", validation_errors);
            StatusCode::BAD_REQUEST
        } else if let Some(json_error) = e.downcast_ref::<SerdeJsonError>() {
            // Handle JSON serialization/deserialization errors
            error_message = format!("JSON deserialize error: {}", json_error);
            warn!("JSON deserialization error: {}", json_error);
            StatusCode::BAD_REQUEST
        } else if let Some(sqlx_error) = e.downcast_ref::<SqlxError>() {
            // Handle SQLx database errors
            error_message = format_sqlx_error(sqlx_error);
            match sqlx_error {
                SqlxError::RowNotFound => {
                    info!("Row not found in the database");
                    StatusCode::NOT_FOUND
                }
                SqlxError::Database(_) => {
                    error!("Database error: {:?}", sqlx_error);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                _ => {
                    warn!("SQLx error: {:?}", sqlx_error);
                    StatusCode::BAD_REQUEST
                }
            }
        } else {
            // Generic error, include a backtrace if available
            error_message = format!("{:?}", e);
            backtrace = Some(format!("{:?}", e.backtrace()));
            error!("Internal server error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        };

    // Create an HTTP response with the determined status code and error details
    let response = HttpResponse::build(status_code).json(&Error {
        message: error_message,
        backtrace,
    });

    // Wrap the original error and the response into an InternalError
    InternalError::from_response(e, response).into()
}

/// Format validation errors
fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut formatted_errors = String::new();

    for (field, field_errors) in errors.field_errors().iter() {
        for error in *field_errors {
            let message = error
                .message
                .clone()
                .unwrap_or_else(|| "Invalid value".into());
            formatted_errors
                .push_str(&format!("Field '{}': {}\n", field, message));
        }
    }

    // Remove the trailing newline character
    formatted_errors.trim_end().to_string()
}

/// Format SQLx errors
fn format_sqlx_error(error: &SqlxError) -> String {
    match error {
        SqlxError::Database(db_error) => {
            // Accéder aux détails pertinents de l'erreur SQLx
            let message = db_error.message();
            let code = db_error
                .code()
                .unwrap_or(std::borrow::Cow::Borrowed("Unknown"));

            // Formatage des informations pertinentes
            format!("Database error: {} (Code: {})", message, code)
        }
        SqlxError::RowNotFound => {
            "No matching rows found in the database.".to_string()
        }
        _ => format!("Database error: {:?}", error),
    }
}
