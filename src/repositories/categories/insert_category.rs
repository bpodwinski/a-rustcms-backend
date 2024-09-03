use sqlx::PgPool;

use crate::dto::category_dto::CategoryId;
use crate::models::categories::categories_table_model::Category;

pub async fn insert_category(
    pool: &PgPool,
    category: Category,
) -> Result<i32, sqlx::Error> {
    let category = sqlx::query_file_as!(
        CategoryId,
        "src/repositories/categories/insert_category.sql",
        category.parent_id,
        category.name,
        category.description
    )
    .fetch_one(pool)
    .await?;

    Ok(category.id)
}
