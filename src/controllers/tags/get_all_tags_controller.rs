use ntex::web::{
    self,
    types::{Query, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::pagination_dto::PaginationParamsDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::tags_service::get_all_tags_service,
};

#[utoipa::path(
  get,
  path = "/tags",
  tag = "Tags",
  params(
    ("page" = Option<i32>, Query, description = "The page number for pagination"),
    ("limit" = Option<i32>, Query, description = "The number of items per page"),
    ("sort_column" = Option<String>, Query, description = "Column to sort by (e.g., 'id', 'name')"),
    ("sort_order" = Option<String>, Query, description = "Sort order ('asc' or 'desc')")
  ),
  responses(
    (status = 200, description = "Get all tags", body = [TagDTO]),
    (status = 404, description = "Tags not found", body = Error),
    (status = 500, description = "Internal Server Error", body = Error)
  ),
)]
#[web::get("/tags")]
pub async fn get_all_tags_controller(
    pool: State<PgPool>,
    params: Query<PaginationParamsDTO>,
) -> Result<HttpResponse, web::Error> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(25);
    let sort_column = params.sort_column.as_deref().unwrap_or("id");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    match get_all_tags_service(
        pool.get_ref(),
        page,
        limit,
        sort_column,
        sort_order,
    )
    .await
    {
        Ok(tags) => Ok(HttpResponse::Ok().json(&tags)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::tags::get_all_tags_controller::get_all_tags_controller;
    use crate::dtos::tag_dto::TagDTO;
    use crate::tests::helpers::setup::{clean_data_test, setup_test_db};
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

        // Clean up test data
        clean_data_test(&pool, "tags", "name", "Test Tag")
            .await
            .expect("Failed to clean up test data");
    }

    #[ntex::test]
    async fn test_get_all_tags_controller_no_tags() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(get_all_tags_controller),
        )
        .await;

        // Clear the tags table to ensure the database is empty
        sqlx::query!("DELETE FROM tags")
            .execute(&pool)
            .await
            .expect("Failed to clear tags table");

        // Ensure the database is empty
        let row_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tags")
            .fetch_one(&pool)
            .await
            .expect("Failed to count tags");

        assert_eq!(row_count.0, 0, "The database is not empty before the test");

        // Act
        let req = test::TestRequest::get().uri("/tags").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = test::read_body(resp).await;
        let response_body: Vec<TagDTO> =
            serde_json::from_slice(&body).expect("Failed to parse JSON");
        assert!(response_body.is_empty(), "Expected an empty array");
    }
}
