use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    handlers::error_handler::ErrorResponse,
    services::tags::delete_tag_by_id_service::delete_tag_by_id_service,
};

#[web::delete("/tags/{id}")]
pub async fn delete_tag_controller(
    pool: web::types::State<PgPool>,
    tag_id: web::types::Path<i32>,
) -> Result<HttpResponse, Error> {
    match delete_tag_by_id_service(pool.get_ref(), tag_id.into_inner()).await {
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
