use anyhow::Result;
use ntex::web::{
    self,
    types::{Json, Path, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::user_dtos::{CreateUserDTO, UserDTO},
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::users_service::update_user_service,
};

#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "Users",
    request_body = CreateUserDTO,
    params(
        ("id" = i32, description = "ID of the user")
    ),
    responses(
        (status = 200, description = "User updated", body = TagDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 404, description = "User not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::put("/users/{id}")]
pub async fn update_user_controller(
    pool: State<PgPool>,
    user_id: Path<i32>,
    user_dto: Json<CreateUserDTO>,
) -> Result<HttpResponse, web::Error> {
    match update_user_service(
        pool.get_ref(),
        user_id.into_inner(),
        user_dto.into_inner(),
    )
    .await
    {
        Ok(updated_user) => {
            Ok(HttpResponse::Ok().json(&UserDTO::from(updated_user)))
        }
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
