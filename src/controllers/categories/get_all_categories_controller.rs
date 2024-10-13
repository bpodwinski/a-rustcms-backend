use ntex::web::{
    self,
    types::{Query, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::pagination_dto::PaginationParamsDTO,
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::categories_service::get_all_categories_service,
};

#[utoipa::path(
  get,
  path = "/categories",
  tag = "Categories",
  params(
    ("page" = Option<i32>, Query, description = "The page number for pagination"),
    ("limit" = Option<i32>, Query, description = "The number of items per page"),
    ("sort_column" = Option<String>, Query, description = "Column to sort by (e.g., 'id', 'name')"),
    ("sort_order" = Option<String>, Query, description = "Sort order ('asc' or 'desc')")
  ),
  security(
    ("api_key" = [])
),
  responses(
    (status = 200, description = "Get all categories", body = CategoryDTO),
    (status = 500, description = "Internal Server Error", body = Error)
  ),
)]
#[web::get("/categories")]
pub async fn get_all_categories_controller(
    pool: State<PgPool>,
    params: Query<PaginationParamsDTO>,
) -> Result<HttpResponse, web::Error> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(25);
    let sort_column = params.sort_column.as_deref().unwrap_or("id");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    match get_all_categories_service(
        pool.get_ref(),
        page,
        limit,
        sort_column,
        sort_order,
    )
    .await
    {
        Ok(categories) => Ok(HttpResponse::Ok().json(&categories)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::controllers::categories::get_all_categories_controller::get_all_categories_controller;
    use crate::dtos::category_dto::CategoryDTO;
    use crate::tests::helpers::setup::setup_test_db;
    use ntex::http;
    use ntex::web::{self, test};

    #[ntex::test]
    async fn test_get_all_categories_controller_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(get_all_categories_controller),
        )
        .await;

        sqlx::query!(
            r#"
            INSERT INTO categories (parent_id, name, slug, description) 
            VALUES ($1, $2, $3, $4)
            "#,
            None::<i32>,
            "Test Category",
            "test-category",
            "Description for test category"
        )
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::get().uri("/categories").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response_body: Vec<CategoryDTO> =
            serde_json::from_slice(&body).expect("Failed to parse JSON");

        let category = response_body.iter().find(|&cat| {
            cat.name == "Test Category" && cat.slug == "test-category"
        });

        assert!(category.is_some(), "Category not found in the response");
        assert_eq!(
            category.unwrap().description.as_deref(),
            Some("Description for test category")
        );

        // Clean Data
        sqlx::query!(
            r#"
            DELETE FROM categories WHERE name = $1
            "#,
            "Test Category"
        )
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
    }
}
