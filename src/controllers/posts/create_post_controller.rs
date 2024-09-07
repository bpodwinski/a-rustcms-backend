use ntex::web::{self, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::models::posts::posts_table_model::Post;
use crate::services::posts::create_post::create_post_service;

#[derive(Deserialize)]
struct CreatePostRequest {
    post: Post,
    categories_ids: Vec<i32>,
}

#[web::post("/posts")]
pub async fn create_post_controller(
    pool: web::types::State<PgPool>,
    request: web::types::Json<CreatePostRequest>,
) -> HttpResponse {
    let CreatePostRequest {
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
