use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::category_dto::CategoryDTO;

pub async fn select(
    pool: &PgPool,
    category_id: i32,
) -> Result<CategoryDTO, sqlx::Error> {
    // Execute SQL query to fetch the post with the specified ID
    let row = sqlx::query_file!(
        "src/repositories/categories/select_category_by_id.sql",
        category_id
    )
    .fetch_one(pool)
    .await?;

    // Map the result to a PostDTO
    let category = CategoryDTO {
        id: Some(row.id),
        parent_id: row.parent_id,
        name: row.name,
        slug: row.slug,
        description: Some(row.description.unwrap_or_else(|| String::new())),
        date_created: Some(row.date_created),
    };

    Ok(category)
}
