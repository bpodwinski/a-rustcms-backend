use ntex::web::{self, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    dtos::post_dto::CreatePostDTO,
    services::posts::create_post_service::create_post_service,
};

#[web::post("/posts")]
pub async fn create_post_controller(
    pool: web::types::State<PgPool>,
    request: web::types::Json<CreatePostDTO>,
) -> HttpResponse {
    let CreatePostDTO {
        post,
        categories_ids,
    } = request.into_inner();

    if let Err(errors) = post.validate() {
        return HttpResponse::BadRequest().json(&errors);
    }

    match create_post_service(pool.get_ref(), post, categories_ids).await {
        Ok(post_with_categories) => {
            HttpResponse::Created().json(&post_with_categories)
        }
        Err(err) => {
            eprintln!("Failed to create post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
