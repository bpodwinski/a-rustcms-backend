use anyhow::Result;
use sqlx::PgPool;

use crate::models::categories::categories_table_model::CategoryModel;

pub async fn insert(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel, sqlx::Error> {
    let category = sqlx::query_file_as!(
        CategoryModel,
        "src/repositories/categories/insert_category.sql",
        category_model.parent_id,
        category_model.name,
        category_model.slug,
        category_model.description
    )
    .fetch_one(pool)
    .await?;

    Ok(category)
}
