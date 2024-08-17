use sqlx::PgPool;

use crate::{
    models::categories::categories_table_model::Category,
    repositories::categories::insert_category::insert_category,
};

pub async fn create_category_service(
    pool: &PgPool,
    category: Category,
) -> Result<i32, sqlx::Error> {
    let category_id = insert_category(pool, category).await?;
    Ok(category_id)
}
