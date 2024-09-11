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

#[cfg(test)]
mod tests {
    use crate::controllers::tags::get_tag_by_id_controller::get_tag_by_id_controller;
    use crate::dtos::tag_dto::TagDTO;
    use crate::tests::helpers::setup::setup_test_db;
    use ntex::http;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_get_all_tags_controller_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(get_tag_by_id_controller),
        )
        .await;

        let inserted_tag = sqlx::query!(
            r#"
            INSERT INTO tags (name, slug, description) 
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            "Test Tag By ID",
            "test-tag-by-id",
            "Description for test tag by id"
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get()
            .uri(&format!("/tags/{}", inserted_tag.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_body: TagDTO =
            serde_json::from_slice(&body).expect("Failed to parse JSON");

        let tag = &response_body;
        assert_eq!(tag.name, "Test Tag By ID");
        assert_eq!(tag.slug, "test-tag-by-id");
        assert_eq!(
            tag.description.as_deref(),
            Some("Description for test tag by id")
        );

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM tags WHERE id = $1
            "#,
            inserted_tag.id
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }
}
