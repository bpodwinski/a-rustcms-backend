use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::tags::get_all_tags_service::get_all_tags_service;

#[web::get("/tags")]
pub async fn get_all_tags_controller(
    pool: web::types::State<PgPool>,
) -> HttpResponse {
    match get_all_tags_service(pool.get_ref()).await {
        Ok(tags) => HttpResponse::Ok().json(&tags),
        Err(err) => {
            eprintln!("Failed to fetch tags: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::tags::get_all_tags_controller::get_all_tags_controller;
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
                .service(get_all_tags_controller),
        )
        .await;

        sqlx::query!(
            r#"
            INSERT INTO tags (name, slug, description) 
            VALUES ($1, $2, $3)
            "#,
            "Test Tag",
            "test-tag",
            "Description for test tag"
        )
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get().uri("/tags").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_body: Vec<TagDTO> =
            serde_json::from_slice(&body).expect("Failed to parse JSON");

        let tag = response_body
            .iter()
            .find(|&tag| tag.name == "Test Tag" && tag.slug == "test-tag");

        assert!(tag.is_some(), "Tag not found in the response");
        assert_eq!(
            tag.unwrap().description.as_deref(),
            Some("Description for test tag")
        );

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM tags WHERE name = $1
            "#,
            "Test Tag"
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }
}
