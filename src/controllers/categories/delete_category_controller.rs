use anyhow::Result;
use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::category_dto::DeleteCategoryIdsDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::categories_service::delete_category_service,
};

#[utoipa::path(
    delete,
    path = "/categories",
    tag = "Categories",
    request_body = DeleteCategoryIdsDTO,
    responses(
        (status = 200, description = "Categories deleted", body = i32),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::delete("/categories")]
pub async fn delete_category_controller(
    pool: State<PgPool>,
    delete_category_ids_dto: Json<DeleteCategoryIdsDTO>,
) -> Result<HttpResponse, web::Error> {
    match delete_category_service(
        pool.get_ref(),
        delete_category_ids_dto.into_inner(),
    )
    .await
    {
        Ok(deleted_ids) => Ok(HttpResponse::Ok().json(&deleted_ids)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
