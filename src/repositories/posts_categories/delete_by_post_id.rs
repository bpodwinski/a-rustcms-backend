use crate::models::posts_categories::posts_categories_table_model::PostsCategories;
use sqlx::PgPool;

/// Deletes entries from the posts_categories table based on the post_id.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post_id` - The ID of the post for which the associated categories
/// should be deleted.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `PostsCategories` records that
/// were deleted, or a `sqlx::Error` if there is an error during the
/// query execution.
///
/// # Errors
///
/// This function will return an error if there is an issue executing
/// the SQL query.
pub async fn delete_post_categories_by_post_id(
    pool: &PgPool,
    post_id: i32,
) -> Result<Vec<PostsCategories>, sqlx::Error> {
    let rows = sqlx::query_file_as!(
        PostsCategories,
        "src/repositories/posts_categories/delete_by_post_id.sql",
        post_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
