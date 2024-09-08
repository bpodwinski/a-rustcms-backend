use anyhow::Result;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, tag_id: i32) -> Result<u64, sqlx::Error> {
    let result =
        sqlx::query_file!("src/repositories/tags/delete_tag_by_id.sql", tag_id)
            .execute(pool)
            .await?;

    Ok(result.rows_affected())
}
