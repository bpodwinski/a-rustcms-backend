use anyhow::Result;
use ntex::web::{
    self,
    types::{Path, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
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
        (status = 404, description = "Category not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::get("/categories/{id}")]
pub async fn get_category_by_id_controller(
    pool: State<PgPool>,
    category_id: Path<i32>,
) -> Result<HttpResponse, web::Error> {
    match get_category_by_id_service(pool.get_ref(), category_id.into_inner())
        .await
    {
        Ok(category) => Ok(HttpResponse::Ok().json(&category)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
