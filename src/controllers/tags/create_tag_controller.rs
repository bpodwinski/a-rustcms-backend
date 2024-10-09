use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::CreateTagDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::tags_service::create_tag_service,
};

#[utoipa::path(
    post,
    path = "/tags",
    tag = "Tags",
    request_body = CreateTagDTO,
    responses(
        (status = 201, description = "Create tag", body = TagDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    ),
)]
#[web::post("/tags")]
pub async fn create_tag_controller(
    pool: State<PgPool>,
    tag_dto: Json<CreateTagDTO>,
) -> Result<HttpResponse, web::Error> {
    match create_tag_service(pool.get_ref(), tag_dto.into_inner()).await {
        Ok(created_tag) => Ok(HttpResponse::Created().json(&created_tag)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::dtos::tag_dto::CreateTagDTO;
    use crate::tests::helpers::setup::setup_test_db;

    #[ntex::test]
    async fn test_create_tag_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_tag_controller),
        )
        .await;

        let tag = CreateTagDTO {
            name: String::from("Test Create Tag Success"),
            slug: String::from("test-create-tag-success"),
            description: None,
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::CREATED);

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM tags WHERE name = $1
            "#,
            "New tag"
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }

    #[ntex::test]
    async fn test_create_tag_validation_failure() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_tag_controller),
        )
        .await;

        let tag = CreateTagDTO {
            name: String::from("Test Validation Failure"),
            slug: String::from("test-validation-failure-*/!"),
            description: None,
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[ntex::test]
    async fn test_create_tag_service_failure() {
        // Arrange
        let invalid_pool = PgPool::connect("postgres://invalid_url").await;
        let app = test::init_service(
            web::App::new()
                .state(invalid_pool.unwrap_err())
                .service(create_tag_controller),
        )
        .await;

        let tag = CreateTagDTO {
            name: String::from("Test Tag Internal Error"),
            slug: String::from("test-tag-internal-error"),
            description: None,
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
