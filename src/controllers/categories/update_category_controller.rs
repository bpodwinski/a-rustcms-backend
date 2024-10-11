use anyhow::Result;
use ntex::web::{
    self,
    types::{Json, Path, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::category_dto::{CategoryDTO, CreateCategoryDTO},
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::categories_service::update_category_service,
};

#[utoipa::path(
    put,
    path = "/categories/{id}",
    tag = "Categories",
    request_body = CreateCategoryDTO,
    params(
        ("id" = i32, description = "ID of the category")
    ),
    responses(
        (status = 200, description = "Category updated", body = CategoryDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 404, description = "Category not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::put("/categories/{id}")]
pub async fn update_category_controller(
    pool: State<PgPool>,
    category_id: Path<i32>,
    category_dto: Json<CreateCategoryDTO>,
) -> Result<HttpResponse, web::Error> {
    match update_category_service(
        pool.get_ref(),
        category_id.into_inner(),
        category_dto.into_inner(),
    )
    .await
    {
        Ok(updated_category) => {
            Ok(HttpResponse::Ok().json(&CategoryDTO::from(updated_category)))
        }
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
