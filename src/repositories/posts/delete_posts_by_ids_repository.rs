use anyhow::Result;
use sqlx::PgPool;

pub async fn delete(
    pool: &PgPool,
    posts_ids: Vec<i32>,
) -> Result<Vec<i32>, sqlx::Error> {
    if posts_ids.is_empty() {
        return Ok(vec![]);
    }

    let result = sqlx::query_file!(
        "src/repositories/posts/delete_posts_by_ids.sql",
        &posts_ids
    )
    .fetch_all(pool)
    .await?;

    let deleted_ids: Vec<i32> =
        result.into_iter().map(|record| record.id).collect();

    Ok(deleted_ids)
}
