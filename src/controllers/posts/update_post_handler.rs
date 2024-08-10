use ntex::web::types::Path;
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::models::posts_model::{NewPostStruct, Status};

/// Update a specific post by its ID.
///
/// This function updates a row in the `posts` table in the database with the
/// specified ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to update.
/// * `updated_post` - A JSON object containing the updated post data.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation. If successful, it returns a `No Content` status.
#[web::put("/posts/{id}")]
pub async fn update_post_by_id(
    pool: web::types::State<PgPool>,
    post_id: Path<i32>,
    updated_post: web::types::Json<NewPostStruct>,
) -> HttpResponse {
    let post_id = post_id.into_inner();
    let updated_post = updated_post.into_inner();

    match sqlx::query!("UPDATE posts SET title = $1, content = $2, author_id = $3, status = $4, date_published = $5 WHERE id = $6",
        updated_post.title,
        updated_post.content,
        updated_post.author_id,
        updated_post.status as Status,
        updated_post.date_published,
        post_id
    )
    .execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error updating post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
