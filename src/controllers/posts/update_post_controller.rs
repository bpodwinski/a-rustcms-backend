use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    dtos::post_dto::{CreatePostDTO, PostDTO},
    handlers::error_to_response_handler::convert_anyhow_to_ntex,
    services::posts_services::update_post_service,
};

#[utoipa::path(
    put,
    path = "/posts/{id}",
    tag = "Posts",
    request_body = CreatePostDTO,
    params(
        ("id" = i32, description = "ID of the post")
    ),
    responses(
        (status = 200, description = "Post updated", body = PostDTO),
        (status = 400, description = "Validation Error", body = Error),
        (status = 404, description = "Post not found", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::put("/posts/{id}")]
pub async fn update_post_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
    post_dto: web::types::Json<CreatePostDTO>,
) -> Result<HttpResponse, web::Error> {
    match update_post_service(
        pool.get_ref(),
        post_id.into_inner(),
        post_dto.into_inner(),
    )
    .await
    {
        Ok(updated_post) => {
            Ok(HttpResponse::Ok().json(&PostDTO::from(updated_post)))
        }
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}
