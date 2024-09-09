use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    handlers::error_handler::ErrorResponse,
    services::tags::get_tags_by_id_service::get_tag_by_id_service,
};

#[web::get("/tags/{id}")]
pub async fn get_tag_by_id_controller(
    pool: web::types::State<PgPool>,
    tag_id: web::types::Path<i32>,
) -> Result<HttpResponse, Error> {
    match get_tag_by_id_service(pool.get_ref(), tag_id.into_inner()).await {
        Ok(Some(tag)) => Ok(HttpResponse::Ok().json(&tag)),
        Ok(None) => Ok(HttpResponse::NotFound().json(&ErrorResponse {
            error: format!("tag not found"),
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
