use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::category_dto::CategoryDTO;

pub async fn select(pool: &PgPool) -> Result<Vec<CategoryDTO>, sqlx::Error> {
    let rows = sqlx::query_file!(
        "src/repositories/categories/select_all_categories.sql"
    )
    .fetch_all(pool)
    .await?;

    let categories = rows
        .into_iter()
        .map(|row| CategoryDTO {
            id: Some(row.id),
            parent_id: row.parent_id,
            name: row.name,
            slug: row.slug,
            description: Some(row.description.unwrap_or_else(|| String::new())),
            date_created: Some(row.date_created),
        })
        .collect();

    Ok(categories)
}
