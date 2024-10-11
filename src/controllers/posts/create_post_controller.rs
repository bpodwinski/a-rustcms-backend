use ntex::web::types::{Json, State};
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    dtos::post_dto::CreatePostDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::posts_services::create_post_service,
};

#[utoipa::path(
    post,
    path = "/posts",
    tag = "Posts",
    request_body = CreatePostDTO,
    responses(
        (status = 201, description = "Post created successfully", body = PostDTO),
        (status = 400, description = "Validation error", body = Error),
        (status = 500, description = "Internal server error", body = Error)
    )
)]
#[web::post("/posts")]
pub async fn create_post_controller(
    pool: State<PgPool>,
    post_dto: Json<CreatePostDTO>,
) -> Result<HttpResponse, web::Error> {
    match create_post_service(pool.get_ref(), post_dto.into_inner()).await {
        Ok(post_with_categories) => {
            Ok(HttpResponse::Created().json(&post_with_categories))
        }
        Err(err) => Err(convert_anyhow_to_ntex(err)),
    }
}
