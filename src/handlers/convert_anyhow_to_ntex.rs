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
    let status_code = if let Some(validation_errors) =
        e.downcast_ref::<ValidationErrors>()
    {
        // Handle validation errors
        error_message = format!("Validation error: {:?}", validation_errors);
        warn!("Validation error occurred: {:?}", validation_errors);
        StatusCode::BAD_REQUEST
    } else if let Some(json_error) = e.downcast_ref::<SerdeJsonError>() {
        // Handle JSON serialization/deserialization errors
        error_message = format!("JSON deserialize error: {}", json_error);
        warn!("JSON deserialization error: {}", json_error);
        StatusCode::BAD_REQUEST
    } else if let Some(sqlx_error) = e.downcast_ref::<SqlxError>() {
        // Handle SQLx database errors
        error_message = format!("Database error: {:?}", sqlx_error);
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
