use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::post_dto::DeletePostIdsDTO,
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::posts_services::delete_post_service,
};

#[utoipa::path(
    delete,
    path = "/posts",
    tag = "Posts",
    request_body = DeletePostIdsDTO,
    responses(
        (status = 200, description = "Posts deleted", body = i32),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::delete("/posts")]
pub async fn delete_post_controller(
    pool: State<PgPool>,
    delete_post_ids_dto: Json<DeletePostIdsDTO>,
) -> Result<HttpResponse, web::Error> {
    match delete_post_service(pool.get_ref(), delete_post_ids_dto.into_inner())
        .await
    {
        Ok(deleted_ids) => Ok(HttpResponse::Ok().json(&deleted_ids)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};
    use serde_json::from_slice;

    use super::*;
    use crate::models::posts_model::PostsStatus;
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

        let inserted_post = sqlx::query!(
            r#"
            INSERT INTO posts (title, content, slug, author_id, status) 
            VALUES ($1, $2, $3, $4, $5::posts_status)
            RETURNING id
            "#,
            "Test Delete Post",
            "Test Delete Post Content",
            "test-delete-post",
            user_id,
            PostsStatus::Published as _
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::delete()
            .uri(&format!("/posts"))
            .set_json(&DeletePostIdsDTO {
                ids: vec![inserted_post.id],
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Check that the response contains the deleted post ID
        let body = test::read_body(resp).await;
        let deleted_ids: DeletePostIdsDTO =
            from_slice(&body).expect("Failed to parse response body");
        assert_eq!(deleted_ids.ids, vec![inserted_post.id]);
    }
}
