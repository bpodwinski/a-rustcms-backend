use ntex::web::types::Path;
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::models::posts_model::{PostStruct, Status};

/// Fetch a specific post by its ID.
///
/// This function queries the `posts` table in the database for a row with the
/// specified ID and returns it as JSON.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to fetch.
///
/// # Returns
///
/// A `HttpResponse` containing the post as JSON or an internal server error if the post is not found.
#[web::get("/posts/{id}")]
pub async fn get_post_by_id(
    pool: web::types::State<PgPool>,
    post_id: Path<i32>,
) -> HttpResponse {
    let post_id = post_id.into_inner();

    match sqlx::query_file_as!(
        PostStruct,
        "src/sql/get_post_by_id.sql",
        post_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(post) => HttpResponse::Ok().json(&post),
        Err(e) => {
            eprintln!("Error fetching post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
