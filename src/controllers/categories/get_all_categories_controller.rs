use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::categories::get_all_categories_service::get_all_categories_service;

#[web::get("/categories")]
pub async fn get_all_categories_controller(
    pool: web::types::State<PgPool>,
) -> HttpResponse {
    match get_all_categories_service(pool.get_ref()).await {
        Ok(categories) => HttpResponse::Ok().json(&categories),
        Err(err) => {
            eprintln!("Failed to fetch categories: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
