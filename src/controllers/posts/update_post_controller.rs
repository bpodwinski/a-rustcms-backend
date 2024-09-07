use ntex::web::{self, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::models::posts::posts_table_model::Post;
use crate::services::posts::update_post::update_post_service;

/// Update an existing post in the database.
///
/// This function updates the specified post in the database
/// with the provided title, content, status, and publication date.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to update.
/// * `updated_post` - A JSON object containing the updated post data.
///
/// # Returns
///
/// A `HttpResponse` containing the updated post in JSON format.
/// If successful, it returns an `Ok` status.
#[web::put("/posts/{id}")]
pub async fn update_post_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
    updated_post: web::types::Json<Post>,
) -> HttpResponse {
    // Validate the incoming data
    if let Err(errors) = updated_post.validate() {
        return HttpResponse::BadRequest().json(&errors);
    }

    // Call the update service
    match update_post_service(
        pool.get_ref(),
        post_id.into_inner(),
        updated_post.title.clone(),
        updated_post.content.clone(),
        updated_post.status.clone(),
        updated_post.date_published,
    )
    .await
    {
        Ok(post) => HttpResponse::Ok().json(&post),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().body("Post not found")
        }
        Err(err) => {
            eprintln!("Failed to update post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
