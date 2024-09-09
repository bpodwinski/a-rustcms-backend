use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags::tags_table_model::TagModel;

pub async fn insert(
    pool: &PgPool,
    tag_model: TagModel,
) -> Result<TagModel, sqlx::Error> {
    let tag = sqlx::query_file_as!(
        TagModel,
        "src/repositories/tags/insert_tag.sql",
        tag_model.name,
        tag_model.slug,
        tag_model.description
    )
    .fetch_one(pool)
    .await?;

    Ok(tag)
}
