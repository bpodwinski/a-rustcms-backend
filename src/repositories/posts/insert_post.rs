use sqlx::{Acquire, Postgres, Transaction};

use crate::{
    dto::post_dto::PostId,
    models::posts::{posts_table_model::Post, posts_type_model::PostsStatus},
};

/// Inserts a new post into the database and returns the generated post ID.
///
/// # Arguments
///
/// * `tx` - A mutable reference to a database transaction. This ensures
/// that the post insertion is part of a larger transactional operation,
/// which can be committed or rolled back as a whole.
/// * `post` - A reference to the `Post` struct containing the data
/// to be inserted.
///
/// # Returns
///
/// Returns a `Result` containing the generated post ID (`i32`) if the
/// insertion is successful, or a `sqlx::Error` if there is an error during
/// the query execution.
///
/// # Errors
///
/// This function will return an error if there is an issue executing the
/// SQL query or acquiring the transaction.
///
pub async fn insert_post_repository(
    tx: &mut Transaction<'_, Postgres>,
    post: &Post,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_file_as!(
        PostId,
        "src/repositories/posts/insert_post.sql",
        post.title,
        post.content,
        post.slug,
        post.author_id,
        post.status.clone() as PostsStatus,
        post.date_published
    )
    .fetch_one(&mut *tx.acquire().await?)
    .await?;

    Ok(result.id)
}
