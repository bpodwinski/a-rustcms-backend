use ntex::web::{
    self,
    types::{Path, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::users_service::get_user_by_id_service,
};

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, description = "ID of the user")
    ),
    responses(
        (status = 200, description = "User retrieved", body = TagDTO),
        (status = 404, description = "User not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::get("/users/{id}")]
pub async fn get_user_by_id_controller(
    pool: State<PgPool>,
    user_id: Path<i32>,
) -> Result<HttpResponse, web::Error> {
    match get_user_by_id_service(pool.get_ref(), user_id.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
