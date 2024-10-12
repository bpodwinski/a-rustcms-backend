use ntex::web::{
    self,
    types::{Json, State},
    Error, HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::user_dtos::DeleteUserIdsDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::users_service::delete_user_by_id_service,
};

#[utoipa::path(
    delete,
    path = "/users",
    tag = "Users",
    request_body = DeleteUserIdsDTO,
    responses(
        (status = 200, description = "Users deleted", body = i32),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::delete("/users")]
pub async fn delete_user_controller(
    pool: State<PgPool>,
    user_id: Json<DeleteUserIdsDTO>,
) -> Result<HttpResponse, Error> {
    match delete_user_by_id_service(pool.get_ref(), user_id.into_inner()).await
    {
        Ok(deleted_ids) => Ok(HttpResponse::Ok().json(&deleted_ids)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
