use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::posts::get_all_posts_service::get_all_posts_service;

#[web::get("/posts")]
pub async fn get_all_posts_controller(
    pool: web::types::State<PgPool>,
) -> HttpResponse {
    match get_all_posts_service(pool.get_ref()).await {
        Ok(posts) => HttpResponse::Ok().json(&posts),
        Err(err) => {
            eprintln!("Failed to fetch posts: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::posts::get_all_posts_controller::get_all_posts_controller;
    use crate::dtos::post_dto::PostDTO;
    use crate::models::posts::posts_type_model::PostsStatus;
    use crate::tests::helpers::setup::setup_test_db;
    use chrono::NaiveDateTime;
    use ntex::http;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_get_all_posts_controller_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(get_all_posts_controller),
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
            "2024-01-01 12:00:00",
            "%Y-%m-%d %H:%M:%S",
        )
        .expect("Failed to parse date");

        let inserted_post = sqlx::query!(
            r#"
            INSERT INTO posts (title, content, slug, author_id, status, date_published) 
            VALUES ($1, $2, $3, $4, $5::posts_status, $6)
            RETURNING id
            "#,
            "Test Post",
            "Test Post Content",
            "test-post",
            user_id,
            PostsStatus::Published as _,
            custom_date
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get().uri("/posts").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_body: Vec<PostDTO> =
            serde_json::from_slice(&body).expect("Failed to parse JSON");

        let post = response_body.iter().find(|&post| {
            post.title == "Test Post" && post.slug == "test-post"
        });

        assert!(post.is_some(), "Post not found in the response");
        assert_eq!(post.unwrap().status, PostsStatus::Published);

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM posts WHERE id = $1
            "#,
            inserted_post.id
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }
}
