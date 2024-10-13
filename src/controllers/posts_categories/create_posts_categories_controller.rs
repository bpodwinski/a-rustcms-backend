use ntex::web::types::{Json, State};
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::dtos::posts_categories_dto::CreatePostsCategoriesDTO;
use crate::handlers::error_to_response_handler::convert_anyhow_to_ntex;
use crate::services::posts_categories_service::create_post_category_service;

#[utoipa::path(
    post,
    path = "/posts-categories",
    tag = "Posts Categories",
    request_body = CreatePostsCategoriesDTO,
    responses(
        (status = 201, description = "Posts Categories created successfully", body = PostsCategoriesDTO),
        (status = 400, description = "Validation error", body = Error),
        (status = 500, description = "Internal server error", body = Error)
    )
)]
#[web::post("/posts-categories")]
pub async fn create_posts_categories_controller(
    pool: State<PgPool>,
    posts_categories_dto: Json<CreatePostsCategoriesDTO>,
) -> Result<HttpResponse, web::Error> {
    match create_post_category_service(
        pool.get_ref(),
        posts_categories_dto.into_inner(),
    )
    .await
    {
        Ok(posts_categories) => {
            Ok(HttpResponse::Created().json(&posts_categories))
        }
        Err(err) => Err(convert_anyhow_to_ntex(err)),
    }
}
