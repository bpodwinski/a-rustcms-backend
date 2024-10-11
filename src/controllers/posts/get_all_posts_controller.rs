use ntex::web::{
    self,
    types::{Query, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::pagination_dto::PaginationParamsDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::posts_services::get_all_posts_service,
};

#[utoipa::path(
    get,
    path = "/posts",
    tag = "Posts",
  params(
    ("page" = Option<i32>, Query, description = "The page number for pagination"),
    ("limit" = Option<i32>, Query, description = "The number of items per page"),
    ("sort_column" = Option<String>, Query, description = "Column to sort by (e.g., 'id', 'name')"),
    ("sort_order" = Option<String>, Query, description = "Sort order ('asc' or 'desc')")
  ),
    responses(
        (status = 200, description = "Get all posts", body = PostDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[web::get("/posts")]
pub async fn get_all_posts_controller(
    pool: State<PgPool>,
    params: Query<PaginationParamsDTO>,
) -> Result<HttpResponse, web::Error> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(25);
    let sort_column = params.sort_column.as_deref().unwrap_or("id");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    match get_all_posts_service(
        pool.get_ref(),
        page,
        limit,
        sort_column,
        sort_order,
    )
    .await
    {
        Ok(posts) => Ok(HttpResponse::Ok().json(&posts)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::posts::get_all_posts_controller::get_all_posts_controller;
    use crate::dtos::post_dto::PostDTO;
    use crate::models::posts_model::PostsStatus;
    use crate::tests::helpers::setup::{clean_data_test, setup_test_db};
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
            PostsStatus::Published as _,
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get()
            .uri("/posts?limit=100&offset=0")
            .to_request();
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

        // Clean up test data
        clean_data_test(&pool, "posts", "name", "Test Post")
            .await
            .expect("Failed to clean up test data");
    }
}
