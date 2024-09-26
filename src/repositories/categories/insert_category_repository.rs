use anyhow::Result;
use sqlx::*;

use crate::{
    models::categories::categories_table_model::CategoryModel,
    repositories::{BindValue, QueryBuilder},
};

/* pub async fn insert(
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
} */

pub async fn insert(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel, Error> {
    let result = QueryBuilder::<CategoryModel>::new(&pool)
        .table("categories")
        .fields(&["parent_id", "name", "slug", "description"])
        .values(vec![
            match category_model.parent_id {
                Some(id) => BindValue::Int(id),
                None => BindValue::Null,
            },
            BindValue::Text(category_model.name),
            BindValue::Text(category_model.slug),
            match category_model.description {
                Some(desc) => BindValue::Text(desc),
                None => BindValue::Null,
            },
        ])
        .insert()
        .await?;

    Ok(result)
}
