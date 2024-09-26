use anyhow::Result;
use sqlx::PgPool;

use crate::{
    models::categories::categories_table_model::CategoryModel,
    repositories::{BindValue, QueryBuilder},
};

pub async fn delete(
    pool: &PgPool,
    category_id: i32,
) -> Result<u64, sqlx::Error> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .delete("category_id", BindValue::Int(category_id))
        .await?;

    Ok(result)
}
