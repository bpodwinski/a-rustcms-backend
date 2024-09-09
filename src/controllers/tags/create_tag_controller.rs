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

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::dtos::tag_dto::CreateTagDTO;
    use crate::tests::helpers::setup_test_db;

    #[ntex::test]
    async fn test_create_tag_success() {
        let pool = setup_test_db().await;

        let tag = CreateTagDTO {
            name: String::from("New tag"),
            slug: String::from("new-tag"),
            description: Some(String::from("Test tag description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_tag_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    #[ntex::test]
    async fn test_create_tag_validation_failure() {
        let pool = setup_test_db().await;

        let tag = CreateTagDTO {
            name: String::new(), // Empty name, should trigger validation failure
            slug: String::from("new-tag"),
            description: Some(String::from("Test tag description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_tag_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[ntex::test]
    async fn test_create_tag_service_failure() {
        let invalid_pool = PgPool::connect("postgres://invalid_url").await;

        let tag = CreateTagDTO {
            name: String::from("New tag"),
            slug: String::from("new-tag"),
            description: Some(String::from("Test tag description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(invalid_pool.unwrap_err())
                .service(create_tag_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
