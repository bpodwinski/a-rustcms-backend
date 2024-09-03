use sqlx::PgPool;

/// Deletes a post from the database by its ID and returns the number
/// of affected rows.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post_id` - The ID of the post to be deleted.
///
/// # Returns
///
/// Returns a `Result` containing the number of rows affected (`u64`) if
/// the deletion is successful, or a `sqlx::Error` if there is an error
/// during the query execution.
///
/// # Errors
///
/// This function will return an error if there is an issue executing the
/// SQL query.
///
pub async fn delete_post_by_id(
    pool: &PgPool,
    post_id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "src/repositories/posts/delete_post_by_id.sql",
        post_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
