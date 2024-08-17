use sqlx::PgPool;

use crate::models::categories::categories_table_model::Category;

#[derive(sqlx::FromRow)]
struct InsertCategoryId {
    id: i32,
}

pub async fn insert_category(
    pool: &PgPool,
    category: Category,
) -> Result<i32, sqlx::Error> {
    let response = sqlx::query_file_as!(
        InsertCategoryId,
        "src/repositories/categories/insert_category.sql",
        category.parent_id,
        category.name,
        category.description
    )
    .fetch_one(pool)
    .await?;

    Ok(response.id)
}
