use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::tags::get_all_tags_service::get_all_tags_service;

#[web::get("/tags")]
pub async fn get_all_tags_controller(
    pool: web::types::State<PgPool>,
) -> HttpResponse {
    match get_all_tags_service(pool.get_ref()).await {
        Ok(tags) => HttpResponse::Ok().json(&tags),
        Err(err) => {
            eprintln!("Failed to fetch tags: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
