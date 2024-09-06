use ntex::web::{self, types::Json, Error, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    dto::tag_dto::TagDTO, services::tags::create_tag::create_tag_service,
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[web::post("/tags")]
pub async fn create_tag_controller(
    pool: web::types::State<PgPool>,
    tag_dto: Result<Json<TagDTO>, web::error::JsonPayloadError>,
) -> Result<HttpResponse, Error> {
    match tag_dto {
        Ok(tag_dto) => {
            match create_tag_service(pool.get_ref(), tag_dto.into_inner()).await
            {
                Ok(created_tag) => {
                    Ok(HttpResponse::Created().json(&created_tag))
                }
                Err(err) => {
                    let error_response = ErrorResponse {
                        error: format!("Error: {:?}", err),
                    };
                    Ok(HttpResponse::BadRequest().json(&error_response))
                }
            }
        }
        Err(err) => {
            let error_response = ErrorResponse {
                error: format!("JSON deserialize error: {:?}", err),
            };
            Ok(HttpResponse::BadRequest().json(&error_response))
        }
    }
}
