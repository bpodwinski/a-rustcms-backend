use anyhow::Result;
use sqlx::PgPool;

pub async fn delete(
    pool: &PgPool,
    category_id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "src/repositories/categories/delete_category_by_id.sql",
        category_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
