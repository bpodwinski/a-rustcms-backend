use crate::repositories::{
    posts::delete_post_by_id::delete_post_by_id,
    posts_categories::delete_by_post_id::delete_post_categories_by_post_id,
};
use sqlx::PgPool;

/// Deletes a post and its associated categories from the database.
///
/// This function first deletes all categories associated with the given post ID,
/// then deletes the post itself. The function returns the number of affected rows
/// from the post deletion operation.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post_id` - The ID of the post to be deleted.
///
/// # Returns
///
/// Returns a `Result` containing the number of rows affected by the post deletion (`u64`),
/// or a `sqlx::Error` if there is an error during the deletion process.
///
/// # Errors
///
/// This function will return an error if there is an issue deleting the categories
/// or the post itself from the database.
///
/// # Example
///
/// ```rust,no_run
/// use sqlx::PgPool;
/// use crate::services::posts::delete_post_service::delete_post_service;
///
/// let pool = PgPool::connect("DATABASE_URL").await.unwrap();
/// let post_id = 1;
pub async fn delete_post_service(
    pool: &PgPool,
    post_id: i32,
) -> Result<u64, sqlx::Error> {
    // Delete associated categories first
    delete_post_categories_by_post_id(pool, post_id).await?;

    // Delete the post itself
    let rows_affected = delete_post_by_id(pool, post_id).await?;

    // Return the number of affected rows from the post deletion
    Ok(rows_affected)
}
