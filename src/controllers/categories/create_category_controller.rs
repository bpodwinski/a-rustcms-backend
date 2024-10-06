use ntex::web::{self, types::Json, HttpResponse};
use sqlx::PgPool;

use crate::dtos::category_dto::CreateCategoryDTO;
use crate::handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex;
use crate::services::categories_service::create_category_service;

#[utoipa::path(
    post,
    path = "/categories",
    tag = "Categories",
    request_body = CreateCategoryDTO,
    responses(
        (status = 201, description = "Create category", body = CategoryDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    ),
)]
#[web::post("/categories")]
pub async fn create_category_controller(
    pool: web::types::State<PgPool>,
    category_dto: Json<CreateCategoryDTO>,
) -> Result<HttpResponse, web::Error> {
    match create_category_service(pool.get_ref(), category_dto.into_inner())
        .await
    {
        Ok(created_category) => {
            Ok(HttpResponse::Created().json(&created_category))
        }
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::dtos::category_dto::CreateCategoryDTO;
    use crate::tests::helpers::setup::setup_test_db;

    #[ntex::test]
    async fn test_create_category_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_category_controller),
        )
        .await;

        let category = CreateCategoryDTO {
            parent_id: Some(10),
            name: String::from("New Category"),
            slug: String::from("new-category"),
            description: Some(String::from("New category description")),
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::CREATED);

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM categories WHERE name = $1
            "#,
            "New Category"
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }

    #[ntex::test]
    async fn test_create_category_validation_failure() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_category_controller),
        )
        .await;

        let category = CreateCategoryDTO {
            parent_id: Some(10),
            name: String::new(), // Empty name, should trigger validation failure
            slug: String::from("new-category"),
            description: Some(String::from("New category description")),
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[ntex::test]
    async fn test_create_category_service_failure() {
        // Arrange
        let invalid_pool = PgPool::connect("postgres://invalid_url").await;
        let app = test::init_service(
            web::App::new()
                .state(invalid_pool.unwrap_err())
                .service(create_category_controller),
        )
        .await;

        let category = CreateCategoryDTO {
            parent_id: Some(10),
            name: String::from("New Category"),
            slug: String::from("new-category"),
            description: Some(String::from("New category description")),
        };

        // Act
        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
