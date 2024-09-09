use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::posts::get_all_posts_service::get_all_posts_service;

#[web::get("/posts")]
pub async fn get_all_posts_controller(
    pool: web::types::State<PgPool>,
) -> HttpResponse {
    match get_all_posts_service(pool.get_ref()).await {
        Ok(posts) => HttpResponse::Ok().json(&posts),
        Err(err) => {
            eprintln!("Failed to fetch posts: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
