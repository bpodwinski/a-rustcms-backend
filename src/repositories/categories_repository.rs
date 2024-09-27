use anyhow::Result;
use sqlx::*;

use crate::{
    models::categories::categories_table_model::CategoryModel,
    repositories::QueryBuilder,
};

use super::Bind;

pub async fn insert_category(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel, Error> {
    let result = QueryBuilder::<CategoryModel>::new(&pool)
        .table("categories")
        .fields(&["parent_id", "name", "slug", "description"])
        .values(vec![
            category_model.parent_id.map_or(Bind::Null, Bind::Int),
            Bind::Text(category_model.name),
            Bind::Text(category_model.slug),
            category_model.description.map_or(Bind::Null, Bind::Text),
        ])
        .insert()
        .await?;

    Ok(result)
}

pub async fn update_category(
    pool: &PgPool,
    id: i32,
    model: CategoryModel,
) -> Result<CategoryModel, Error> {
    let result = QueryBuilder::<CategoryModel>::new(&pool)
        .table("categories")
        .fields(&["parent_id", "name", "slug", "description"])
        .values(vec![
            model.parent_id.map_or(Bind::Null, Bind::Int),
            Bind::Text(model.name),
            Bind::Text(model.slug),
            model.description.map_or(Bind::Null, Bind::Text),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

pub async fn select_categories(
    pool: &PgPool,
) -> Result<Vec<CategoryModel>, Error> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .fields(&[
            "id",
            "parent_id",
            "name",
            "slug",
            "description",
            "date_created",
        ])
        .select(None, None)
        .await;

    result
}

pub async fn select_category_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<CategoryModel, Error> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .fields(&[
            "id",
            "parent_id",
            "name",
            "slug",
            "description",
            "date_created",
        ])
        .select_one(Some("id"), Some(&Bind::Int(id)))
        .await?;

    Ok(result)
}

pub async fn delete_category_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .delete("id", Bind::Int(id))
        .await?;

    Ok(result)
}
