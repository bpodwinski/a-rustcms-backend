use anyhow::Result;
use sqlx::PgPool;

pub async fn delete(
    pool: &PgPool,
    posts_ids: Vec<i32>,
) -> Result<u64, sqlx::Error> {
    if posts_ids.is_empty() {
        return Ok(0);
    }

    let result = sqlx::query_file!(
        "src/repositories/posts/delete_posts_by_ids.sql",
        &posts_ids
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
