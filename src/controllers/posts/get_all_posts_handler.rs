use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::models::posts_model::{PostStruct, Status};

/// Fetch all posts from the database.
///
/// This function queries the `posts` table in the database and returns all the
/// rows as a JSON array.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
///
/// # Returns
///
/// A `HttpResponse` containing a JSON array of all posts or an internal server error.
#[web::get("/posts")]
pub async fn get_posts(pool: web::types::State<PgPool>) -> HttpResponse {
    let posts =
        match sqlx::query_file_as!(PostStruct, "src/sql/get_all_posts.sql")
            .fetch_all(pool.get_ref())
            .await
        {
            Ok(posts) => posts,
            Err(e) => {
                eprintln!("Error fetching posts: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

    HttpResponse::Ok().json(&posts)
}
