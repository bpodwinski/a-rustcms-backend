use anyhow::Result;
use ntex::web::{self, types::Json, HttpResponse};
use sqlx::PgPool;
use validator::ValidationErrors;

use crate::{
    dtos::category_dto::CategoryDTO,
    middlewares::error_middleware::ErrorResponse,
    services::categories_service::update_category_service,
};

#[utoipa::path(
    put,
    path = "/categories/{id}",
    tag = "Categories",
    request_body = CategoryDTO,
    responses(
        (status = 200, description = "Category updated", body = CategoryDTO),
        (status = 400, description = "Validation Error", body = ErrorResponse),
        (status = 404, description = "Category not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[web::put("/categories/{id}")]
pub async fn update_category_controller(
    pool: web::types::State<PgPool>,
    category_id: web::types::Path<i32>,
    category_dto: Json<CategoryDTO>,
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
        Err(e) => {
            if let Some(_) = e.downcast_ref::<ValidationErrors>() {
                Ok(HttpResponse::BadRequest().json(&ErrorResponse::new(
                    "validation_error",
                    Some("Invalid data provided"),
                    None,
                )))
            } else {
                Ok(HttpResponse::InternalServerError().json(
                    &ErrorResponse::new(
                        "server_error",
                        Some("An internal server error occurred"),
                        None,
                    ),
                ))
            }
        }
    }
}
