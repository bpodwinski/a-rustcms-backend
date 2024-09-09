use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    handlers::error_handler::ErrorResponse,
    services::posts::get_post_by_id_service::get_post_by_id_service,
};

#[web::get("/posts/{id}")]
pub async fn get_post_by_id_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> Result<HttpResponse, Error> {
    match get_post_by_id_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(&post)),
        Ok(None) => Ok(HttpResponse::NotFound().json(&ErrorResponse {
            error: format!("Post not found"),
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
