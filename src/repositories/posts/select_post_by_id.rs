use sqlx::PgPool;

use crate::{
    dto::post_dto::PostDTO, models::posts::posts_type_model::PostsStatus,
};

/// Retrieves single post by its ID from the database and maps it to a PostDTO.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post_id` - The ID of the post to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing a `PostDTO` if the post is found, or a
/// `sqlx::Error` if there is an error during the query.
///
/// # Errors
///
/// This function will return an error if the post with the specified
/// `post_id`does not exist, or if there is an issue executing the SQL query.
///
pub async fn select_post_by_id(
    pool: &PgPool,
    post_id: i32,
) -> Result<PostDTO, sqlx::Error> {
    // Execute SQL query to fetch the post with the specified ID
    let row = sqlx::query_file!(
        "src/repositories/posts/select_post_by_id.sql",
        post_id
    )
    .fetch_one(pool)
    .await?;

    // Map the result to a PostDTO
    let post = PostDTO {
        id: Some(row.id),
        title: row.title,
        content: row.content,
        author_id: row.author_id,
        status: PostsStatus::from(row.status),
        date_published: row.date_published,
        date_created: Some(row.date_created),
        categories: row.categories,
    };

    Ok(post)
}
