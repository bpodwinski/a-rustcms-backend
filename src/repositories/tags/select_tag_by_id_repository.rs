use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags::tags_table_model::TagModel;

pub async fn select(
    pool: &PgPool,
    tag_id: i32,
) -> Result<TagModel, sqlx::Error> {
    let tag_row =
        sqlx::query_file!("src/repositories/tags/select_tag_by_id.sql", tag_id)
            .fetch_one(pool)
            .await?;

    let tag = TagModel {
        id: Some(tag_row.id),
        name: tag_row.name,
        slug: tag_row.slug,
        description: Some(tag_row.description.unwrap_or_default()),
        date_created: Some(tag_row.date_created),
    };

    Ok(tag)
}
