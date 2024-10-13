use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::auth_dtos::LoginRequestDTO,
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::auth_service::login_service,
};

#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth",
    request_body = LoginRequestDTO,
    responses(
        (status = 200, description = "Login successful", body = TokenDTO),
        (status = 401, description = "Invalid credentials", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    ),
)]
#[web::post("/login")]
pub async fn login_controller(
    pool: State<PgPool>,
    login: Json<LoginRequestDTO>,
) -> Result<HttpResponse, web::Error> {
    match login_service(pool.get_ref(), &login.email, &login.password).await {
        Ok(Some(token)) => Ok(HttpResponse::Ok().json(&token)),
        Ok(None) => Ok(HttpResponse::Unauthorized().finish()),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
