use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::services::posts::get_all_posts::get_all_posts_service;

/// Get all posts from the database.
///
/// This function retrieves all rows from the `posts` table in the database.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
///
/// # Returns
///
/// A `HttpResponse` containing the list of all posts in JSON format.
/// If successful, it returns an `Ok` status.
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
