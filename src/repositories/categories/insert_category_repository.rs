use anyhow::Result;
use sqlx::*;

use crate::{
    models::categories::categories_table_model::CategoryModel,
    repositories::{BindValue, QueryBuilder},
};

pub async fn insert(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel, Error> {
    let result = QueryBuilder::<CategoryModel>::new(&pool)
        .table("categories")
        .fields(&["parent_id", "name", "slug", "description"])
        .values(vec![
            category_model
                .parent_id
                .map_or(BindValue::Null, BindValue::Int),
            BindValue::Text(category_model.name),
            BindValue::Text(category_model.slug),
            category_model
                .description
                .map_or(BindValue::Null, BindValue::Text),
        ])
        .insert()
        .await?;

    Ok(result)
}
