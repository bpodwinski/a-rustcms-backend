use anyhow::Result;
use sqlx::PgPool;

use crate::repositories::tags_repository::delete_tag_by_id;

pub async fn delete_tag_by_id_service(
    pool: &PgPool,
    tag_id: i32,
) -> Result<u64, sqlx::Error> {
    let result = delete_tag_by_id(pool, tag_id).await?;

    Ok(result)
}
