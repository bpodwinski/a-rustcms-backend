use sqlx::PgPool;

use crate::{
    dto::category_dto::CategoryId,
    models::categories::categories_table_model::Category,
};

pub async fn insert_category(
    pool: &PgPool,
    category: Category,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_file_as!(
        CategoryId,
        "src/repositories/categories/insert_category.sql",
        category.parent_id,
        category.name,
        category.slug,
        category.description
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
