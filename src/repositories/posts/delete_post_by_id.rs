use sqlx::PgPool;

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
