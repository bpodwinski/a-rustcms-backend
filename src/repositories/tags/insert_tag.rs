use sqlx::PgPool;

use crate::dto::tag_dto::{TagDTO, TagId};

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
pub async fn insert_tag(
    pool: &PgPool,
    tag: &TagDTO,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_file_as!(
        TagId,
        "src/repositories/tags/insert_tag.sql",
        tag.name,
        tag.slug,
        tag.description
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
