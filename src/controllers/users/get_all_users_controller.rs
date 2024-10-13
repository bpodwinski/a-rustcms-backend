use ntex::web::{
    self,
    types::{Query, State},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{
    dtos::pagination_dto::PaginationParamsDTO,
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::users_service::get_all_users_service,
};

#[utoipa::path(
  get,
  path = "/users",
  tag = "Users",
  params(
    ("page" = Option<i32>, Query, description = "The page number for pagination"),
    ("limit" = Option<i32>, Query, description = "The number of items per page"),
    ("sort_column" = Option<String>, Query, description = "Column to sort by (e.g., 'id', 'username')"),
    ("sort_order" = Option<String>, Query, description = "Sort order ('asc' or 'desc')")
  ),
  responses(
    (status = 200, description = "Get all users", body = [UserDTO]),
    (status = 404, description = "Users not found", body = Error),
    (status = 500, description = "Internal Server Error", body = Error)
  ),
)]
#[web::get("/users")]
pub async fn get_all_users_controller(
    pool: State<PgPool>,
    params: Query<PaginationParamsDTO>,
) -> Result<HttpResponse, web::Error> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(25);
    let sort_column = params.sort_column.as_deref().unwrap_or("id");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    match get_all_users_service(
        pool.get_ref(),
        page,
        limit,
        sort_column,
        sort_order,
    )
    .await
    {
        Ok(users) => Ok(HttpResponse::Ok().json(&users)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
