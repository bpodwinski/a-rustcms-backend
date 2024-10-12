use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::user_dtos::CreateUserDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::users_service::create_user_service,
};

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUserDTO,
    responses(
        (status = 201, description = "Create user", body = UserDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    ),
)]
#[web::post("/users")]
pub async fn create_user_controller(
    pool: State<PgPool>,
    user_dto: Json<CreateUserDTO>,
) -> Result<HttpResponse, web::Error> {
    match create_user_service(pool.get_ref(), user_dto.into_inner()).await {
        Ok(created_user) => Ok(HttpResponse::Created().json(&created_user)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
