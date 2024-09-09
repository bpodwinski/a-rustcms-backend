use ntex::web::{
    self, error::JsonPayloadError, types::Json, Error, HttpResponse,
};
use sqlx::PgPool;

use crate::dtos::category_dto::CreateCategoryDTO;
use crate::handlers::error_handler::ErrorResponse;
use crate::services::categories::create_category_service::create_category_service;

#[web::post("/categories")]
pub async fn create_category_controller(
    pool: web::types::State<PgPool>,
    category_dto: Result<Json<CreateCategoryDTO>, JsonPayloadError>,
) -> Result<HttpResponse, Error> {
    match category_dto {
        Ok(category_dto) => {
            match create_category_service(
                pool.get_ref(),
                category_dto.into_inner(),
            )
            .await
            {
                Ok(created_category) => {
                    Ok(HttpResponse::Created().json(&created_category))
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
    use dotenv::dotenv;
    use ntex::http;
    use ntex::web::{self, test};
    use sqlx::{Executor, Pool, Postgres};

    use super::*;
    use crate::db;
    use crate::dtos::category_dto::CreateCategoryDTO;

    async fn setup_test_db() -> Pool<Postgres> {
        dotenv().ok();
        let pool = db::init_pool().await.expect("Failed to create pool");

        pool.execute(
            r#"
            CREATE TEMPORARY TABLE categories (
                id SERIAL PRIMARY KEY,
                parent_id INTEGER,
                name VARCHAR NOT NULL,
                slug VARCHAR NOT NULL,
                description TEXT,
                date_created TIMESTAMP NOT NULL DEFAULT NOW()
            );
            "#,
        )
        .await
        .unwrap();

        pool
    }

    #[ntex::test]
    async fn test_create_category_success() {
        let pool = setup_test_db().await;

        let category = CreateCategoryDTO {
            name: String::from("New Category"),
            slug: String::from("new-category"),
            description: Some(String::from("Test category description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_category_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    #[ntex::test]
    async fn test_create_category_validation_failure() {
        let pool = setup_test_db().await;

        let category = CreateCategoryDTO {
            name: String::new(), // Empty name, should trigger validation failure
            slug: String::from("new-category"),
            description: Some(String::from("Test category description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(create_category_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[ntex::test]
    async fn test_create_category_service_failure() {
        let invalid_pool = PgPool::connect("postgres://invalid_url").await;

        let category = CreateCategoryDTO {
            name: String::from("New Category"),
            slug: String::from("new-category"),
            description: Some(String::from("Test category description")),
        };

        let app = test::init_service(
            web::App::new()
                .state(invalid_pool.unwrap_err())
                .service(create_category_controller),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/categories")
            .set_json(&category)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
