use ntex::web::{
    self, error::JsonPayloadError, types::Json, Error, HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::TagDTO, handlers::error_handler::ErrorResponse,
    services::tags::create_tag_service::create_tag_service,
};

#[web::post("/tags")]
pub async fn create_tag_controller(
    pool: web::types::State<PgPool>,
    tag_dto: Result<Json<TagDTO>, JsonPayloadError>,
) -> Result<HttpResponse, Error> {
    match tag_dto {
        Ok(tag_dto) => {
            match create_tag_service(pool.get_ref(), tag_dto.into_inner()).await
            {
                Ok(created_tag) => {
                    Ok(HttpResponse::Created().json(&created_tag))
                }
                Err(service_error) => {
                    let error_response = service_error.to_error_response();
                    Ok(HttpResponse::BadRequest().json(&error_response))
                }
            }
        }
        Err(err) => {
            let error_response = ErrorResponse {
                error: format!("JSON parse error: {}", err),
                details: None,
            };
            Ok(HttpResponse::BadRequest().json(&error_response))
        }
    }
}
