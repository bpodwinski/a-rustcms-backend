use sqlx::PgPool;

use crate::{
    models::categories::categories_table_model::Category,
    repositories::categories::insert_category_repository,
};

pub async fn create_category_service(
    pool: &PgPool,
    category: Category,
) -> Result<i32, sqlx::Error> {
    let category_id =
        insert_category_repository::insert(pool, category).await?;
    Ok(category_id)
}
