use anyhow::Result;
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    middlewares::error_middleware::ErrorResponse,
    services::categories_service::get_category_by_id_service,
};

#[utoipa::path(
    get,
    path = "/categories/{id}",
    tag = "Categories",
    params(
        ("id" = i32, description = "ID of the category")
    ),
    responses(
        (status = 200, description = "Category retrieved", body = CategoryDTO),
        (status = 404, description = "Category not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[web::get("/categories/{id}")]
pub async fn get_category_by_id_controller(
    pool: web::types::State<PgPool>,
    category_id: web::types::Path<i32>,
) -> Result<HttpResponse, web::Error> {
    match get_category_by_id_service(pool.get_ref(), category_id.into_inner())
        .await
    {
        Ok(category) => Ok(HttpResponse::Ok().json(&category)),
        Err(_) => Ok(HttpResponse::NotFound().json(&ErrorResponse::new(
            "not_found",
            Some("Category not found"),
            None,
        ))),
    }
}
