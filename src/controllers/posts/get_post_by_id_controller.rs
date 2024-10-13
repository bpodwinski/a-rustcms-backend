use ntex::web::{self, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::posts_services::get_post_by_id_service,
};

#[utoipa::path(
    get,
    path = "/posts/{id}",
    tag = "Posts",
    params(
        ("id" = i32, description = "ID of the post")
    ),
    responses(
        (status = 200, description = "Post retrieved", body = PostDTO),
        (status = 404, description = "Post not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::get("/posts/{id}")]
pub async fn get_post_by_id_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> Result<HttpResponse, Error> {
    match get_post_by_id_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(post) => Ok(HttpResponse::Ok().json(&post)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::posts::get_post_by_id_controller::get_post_by_id_controller;
    use crate::dtos::post_dto::PostDTO;
    use crate::models::posts_model::PostsStatus;
    use crate::tests::helpers::setup::setup_test_db;
    use chrono::NaiveDateTime;
    use ntex::http;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_post_by_id_controller_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(get_post_by_id_controller),
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

        let inserted_post = sqlx::query!(
            r#"
            INSERT INTO posts (title, content, slug, author_id, status) 
            VALUES ($1, $2, $3, $4, $5::posts_status)
            RETURNING id
            "#,
            "Test Post",
            "Test Post Content",
            "test-post",
            user_id,
            PostsStatus::Published as _
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get()
            .uri(&format!("/posts/{}", inserted_post.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_body: PostDTO =
            serde_json::from_slice(&body).expect("Failed to parse JSON");

        let post = &response_body;
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.slug, "test-post");
        assert_eq!(post.status, PostsStatus::Published);

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
