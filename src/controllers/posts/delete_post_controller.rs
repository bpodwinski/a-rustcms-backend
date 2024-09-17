use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    dtos::post_dto::DeletePostsDTO, handlers::error_handler::ErrorResponse,
    services::posts::delete_posts_service::delete_posts_service,
};

#[web::delete("/posts")]
pub async fn delete_post_controller(
    pool: web::types::State<PgPool>,
    posts_request: web::types::Json<DeletePostsDTO>,
) -> Result<HttpResponse, Error> {
    let posts_ids = posts_request.into_inner().posts_ids;

    match delete_posts_service(pool.get_ref(), posts_ids).await {
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
    use chrono::NaiveDateTime;
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::models::posts::posts_type_model::PostsStatus;
    use crate::tests::helpers::setup::setup_test_db;

    #[ntex::test]
    async fn test_delete_post_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(delete_post_controller),
        )
        .await;

        // Check if a user already exists
        let existing_user_id: Option<i32> = sqlx::query_scalar!(
            r#"
            SELECT id FROM users LIMIT 1
            "#
        )
        .fetch_optional(&pool)
        .await
        .expect("Failed to query users");

        // If no user exists, create one
        let user_id = if let Some(id) = existing_user_id {
            id
        } else {
            sqlx::query!(
                r#"
                INSERT INTO users (username, email) 
                VALUES ($1, $2)
                RETURNING id
                "#,
                "testuser",
                "testuser@example.com"
            )
            .fetch_one(&pool)
            .await
            .expect("Failed to insert test user")
            .id
        };

        let custom_date = NaiveDateTime::parse_from_str(
            "2024-02-01 06:00:00",
            "%Y-%m-%d %H:%M:%S",
        )
        .expect("Failed to parse date");

        let inserted_post = sqlx::query!(
            r#"
            INSERT INTO posts (title, content, slug, author_id, status, date_published) 
            VALUES ($1, $2, $3, $4, $5::posts_status, $6)
            RETURNING id
            "#,
            "Test Delete Post",
            "Test Delete Post Content",
            "test-delete-post",
            user_id,
            PostsStatus::Published as _,
            custom_date
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::delete()
            .uri(&format!("/posts"))
            .set_json(&DeletePostsDTO {
                posts_ids: vec![inserted_post.id],
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
    }
}
