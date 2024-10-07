use anyhow::Result;
use sqlx::PgPool;

use crate::models::categories_model::CategoryModel;

use super::{Bind, QueryBuilder};

pub async fn insert_category(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel> {
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
) -> Result<CategoryModel> {
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

pub async fn select_categories(pool: &PgPool) -> Result<Vec<CategoryModel>> {
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
        .await?;

    Ok(result)
}

pub async fn select_category_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<CategoryModel> {
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
    ids: Vec<i32>,
) -> Result<Vec<i32>> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .delete("id", ids)
        .await?;

    Ok(result)
}
