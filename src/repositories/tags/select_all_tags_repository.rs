use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags::tags_table_model::TagModel;

pub async fn select(pool: &PgPool) -> Result<Vec<TagModel>, sqlx::Error> {
    let rows = sqlx::query_file!("src/repositories/tags/select_all_tags.sql")
        .fetch_all(pool)
        .await?;

    let tags = rows
        .into_iter()
        .map(|row| TagModel {
            id: Some(row.id),
            name: row.name,
            slug: row.slug,
            description: Some(row.description.unwrap_or_default()),
            date_created: Some(row.date_created),
        })
        .collect();

    Ok(tags)
}
