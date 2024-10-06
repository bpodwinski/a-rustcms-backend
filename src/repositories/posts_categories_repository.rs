use anyhow::Result;
use sqlx::*;

use crate::{
    models::posts_categories::posts_categories_table_model::PostsCategoriesModel,
    repositories::QueryBuilder,
};

use super::Bind;

pub async fn insert_post_category(
    pool: &PgPool,
    model: PostsCategoriesModel,
) -> Result<PostsCategoriesModel, Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(&pool)
        .table("posts_categories")
        .fields(&["post_id", "category_id"])
        .values(vec![Bind::Int(model.post_id), Bind::Int(model.category_id)])
        .insert()
        .await?;

    Ok(result)
}

pub async fn update_post_category(
    pool: &PgPool,
    id: i32,
    model: PostsCategoriesModel,
) -> Result<PostsCategoriesModel, Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(&pool)
        .table("posts_categories")
        .fields(&["post_id", "category_id"])
        .values(vec![Bind::Int(model.post_id), Bind::Int(model.category_id)])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

pub async fn select_posts_categories(
    pool: &PgPool,
) -> Result<Vec<PostsCategoriesModel>, Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(pool)
        .table("posts_categories")
        .fields(&["id", "post_id", "category_id", "date_created"])
        .select(None, None)
        .await;

    result
}

pub async fn select_post_category_by_post_id(
    pool: &PgPool,
    id: i32,
) -> Result<PostsCategoriesModel, Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(pool)
        .table("posts_categories")
        .fields(&["id", "post_id", "category_id", "date_created"])
        .select_one(Some("post_id"), Some(&Bind::Int(id)))
        .await?;

    Ok(result)
}

pub async fn delete_post_category_by_post_id(
    pool: &PgPool,
    post_ids: Vec<i32>,
) -> Result<Vec<i32>, sqlx::Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(pool)
        .table("posts_categories")
        .delete("post_id", post_ids)
        .await?;

    Ok(result)
}

pub async fn delete_post_category_by_category_id(
    pool: &PgPool,
    category_ids: Vec<i32>,
) -> Result<Vec<i32>, sqlx::Error> {
    let result = QueryBuilder::<PostsCategoriesModel>::new(pool)
        .table("posts_categories")
        .delete("category_id", category_ids)
        .await?;

    Ok(result)
}
