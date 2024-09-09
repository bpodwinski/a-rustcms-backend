use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    handlers::error_handler::ErrorResponse,
    services::posts::delete_post_service::delete_post_service,
};

#[web::delete("/posts/{id}")]
pub async fn delete_post_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> Result<HttpResponse, Error> {
    match delete_post_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            Ok(HttpResponse::NoContent().finish())
        }
        Ok(err) => Ok(HttpResponse::NotFound().json(&ErrorResponse {
            error: format!("Not found: {}", err),
            details: None,
        })),
        Err(err) => {
            let error_response = ErrorResponse {
                error: format!("JSON parse error: {}", err),
                details: None,
            };
            Ok(HttpResponse::BadRequest().json(&error_response))
        }
    }
}
