use anyhow::Result;
use ntex::web::{
    self,
    types::{Json, Path, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::{CreateTagDTO, TagDTO},
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::tags_service::update_tag_service,
};

#[utoipa::path(
    put,
    path = "/tags/{id}",
    tag = "Tags",
    request_body = CreateTagDTO,
    params(
        ("id" = i32, description = "ID of the tag")
    ),
    responses(
        (status = 200, description = "Tag updated", body = TagDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 404, description = "Tag not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::put("/tags/{id}")]
pub async fn update_tag_controller(
    pool: State<PgPool>,
    tag_id: Path<i32>,
    tag_dto: Json<CreateTagDTO>,
) -> Result<HttpResponse, web::Error> {
    match update_tag_service(
        pool.get_ref(),
        tag_id.into_inner(),
        tag_dto.into_inner(),
    )
    .await
    {
        Ok(updated_tag) => {
            Ok(HttpResponse::Ok().json(&TagDTO::from(updated_tag)))
        }
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controllers::tags::create_tag_controller::create_tag_controller;
    use crate::dtos::tag_dto::CreateTagDTO;
    use crate::tests::helpers::setup::{clean_data_test, setup_test_db};
    use ntex::http;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_update_tag_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_tag_controller)
                .service(update_tag_controller),
        )
        .await;

        let new_tag = CreateTagDTO {
            name: String::from("Test Update Tag Success"),
            slug: String::from("test-update-tag-success"),
            description: None,
        };

        // Insert a tag first
        let req = test::TestRequest::post()
            .uri("/tags")
            .set_json(&new_tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify the response status
        assert_eq!(
            resp.status(),
            http::StatusCode::CREATED,
            "Tag creation failed"
        );

        // Extract the ID from the response
        let body = test::read_body(resp).await;
        let body_json: TagDTO =
            serde_json::from_slice(&body).expect("Failed to parse JSON");
        let tag_id = body_json.id.expect("Tag ID should be present");

        // Act
        let updated_tag = CreateTagDTO {
            name: String::from("Test Updated Tag New Name"),
            slug: String::from("test-updated-tag-new-name"),
            description: Some(String::from("Updated description")),
        };

        let req = test::TestRequest::put()
            .uri(&format!("/tags/{}", tag_id))
            .set_json(&updated_tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        let body = test::read_body(resp).await;
        let updated_tag_response: TagDTO =
            serde_json::from_slice(&body).expect("Failed to parse JSON");
        assert_eq!(updated_tag_response.name, "Test Updated Tag New Name");
        assert_eq!(updated_tag_response.slug, "test-updated-tag-new-name");
        assert_eq!(
            updated_tag_response.description.unwrap(),
            "Updated description"
        );

        // Clean up test data
        clean_data_test(&pool, "tags", "name", "Test Updated Tag New Name")
            .await
            .expect("Failed to clean up test data");
    }

    #[ntex::test]
    async fn test_update_tag_not_found() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(update_tag_controller),
        )
        .await;

        let updated_tag = CreateTagDTO {
            name: String::from("Non-existent Tag"),
            slug: String::from("non-existent-tag"),
            description: Some(String::from("Description of non-existent tag")),
        };

        // Act
        let req = test::TestRequest::put()
            .uri("/tags/999")
            .set_json(&updated_tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[ntex::test]
    async fn test_update_tag_validation_failure() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(update_tag_controller),
        )
        .await;

        let invalid_tag = CreateTagDTO {
            name: String::from(""),
            slug: String::from("invalid-tag-*"),
            description: None,
        };

        // Act
        let req = test::TestRequest::put()
            .uri("/tags/1")
            .set_json(&invalid_tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[ntex::test]
    async fn test_update_tag_service_failure() {
        // Arrange
        let invalid_pool = PgPool::connect("postgres://invalid_url").await;
        let app = test::init_service(
            web::App::new()
                .state(invalid_pool.unwrap_err())
                .service(update_tag_controller),
        )
        .await;

        let valid_tag = CreateTagDTO {
            name: String::from("Valid Tag"),
            slug: String::from("valid-tag"),
            description: Some(String::from("Description of valid tag")),
        };

        // Act
        let req = test::TestRequest::put()
            .uri("/tags/1")
            .set_json(&valid_tag)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
