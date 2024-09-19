use sqlx::PgPool;

pub async fn select(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let total_items: Option<i64> = sqlx::query_file_scalar!(
        "src/repositories/posts/count_total_posts.sql"
    )
    .fetch_one(pool)
    .await?;

    Ok(total_items.unwrap_or(0))
}
