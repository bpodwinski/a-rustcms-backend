use ntex::web::types::Path;
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

/// Delete a specific post by its ID.
///
/// This function deletes a row from the `posts` table in the database with the
/// specified ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to delete.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation. If successful, it returns a `No Content` status.
#[web::delete("/posts/{id}")]
pub async fn delete_post_by_id(
    pool: web::types::State<PgPool>,
    post_id: Path<i32>,
) -> HttpResponse {
    let post_id = post_id.into_inner();

    match sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
