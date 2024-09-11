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

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::tests::helpers::setup::setup_test_db;

    #[ntex::test]
    async fn test_delete_tag_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(delete_tag_controller),
        )
        .await;

        let inserted_tag = sqlx::query!(
            r#"
            INSERT INTO tags (name, slug, description) 
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            "Test Delete Tag",
            "test-delete-tag",
            "Description for test delete tag"
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::delete()
            .uri(&format!("/tags/{}", inserted_tag.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
    }
}
