use ntex::web::{self, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::models::posts::posts_table_model::Post;
use crate::services::posts::create_post::create_post_service;

/// Create a new post in the database.
///
/// This function inserts a new row into the `posts` table in the database
/// with the provided title, content, and author ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `new_post` - A JSON object containing the new post data.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation.
/// If successful, it returns a `Created` status.
#[web::post("/posts")]
pub async fn create_post_controller(
    pool: web::types::State<PgPool>,
    new_post: web::types::Json<Post>,
) -> HttpResponse {
    if let Err(errors) = new_post.validate() {
        return HttpResponse::BadRequest().json(&errors);
    }

    match create_post_service(pool.get_ref().clone(), new_post.into_inner())
        .await
    {
        Ok(post) => HttpResponse::Created().json(&post),
        Err(err) => {
            eprintln!("Failed to create post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
