use anyhow::Result;
use sqlx::*;

use crate::{
    models::posts::posts_table_model::PostModel, repositories::QueryBuilder,
};

use super::Bind;

pub async fn insert_post(
    pool: &PgPool,
    model: PostModel,
) -> Result<PostModel, Error> {
    let result = QueryBuilder::<PostModel>::new(&pool)
        .table("posts")
        .fields(&[
            "title",
            "content",
            "slug",
            "author_id",
            "status",
            "date_published",
        ])
        .values(vec![
            Bind::Text(model.title),
            Bind::Text(model.content),
            Bind::Text(model.slug),
            Bind::Int(model.author_id),
        ])
        .insert()
        .await?;

    Ok(result)
}

pub async fn update_post(
    pool: &PgPool,
    id: i32,
    model: PostModel,
) -> Result<PostModel, Error> {
    let result = QueryBuilder::<PostModel>::new(&pool)
        .table("posts")
        .fields(&[
            "title",
            "content",
            "slug",
            "author_id",
            "status",
            "date_published",
        ])
        .values(vec![
            Bind::Text(model.title),
            Bind::Text(model.content),
            Bind::Text(model.slug),
            Bind::Int(model.author_id),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

pub async fn select_posts(pool: &PgPool) -> Result<Vec<PostModel>, Error> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .fields(&[
            "id",
            "title",
            "content",
            "slug",
            "author_id",
            "status",
            "date_published",
        ])
        .select(None, None)
        .await;

    result
}

pub async fn select_post_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<PostModel, Error> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .fields(&[
            "id",
            "title",
            "content",
            "slug",
            "author_id",
            "status",
            "date_published",
        ])
        .select_one(Some("id"), Some(&Bind::Int(id)))
        .await?;

    Ok(result)
}

pub async fn delete_post_by_post_id(
    pool: &PgPool,
    ids: Vec<i32>,
) -> Result<Vec<i32>, sqlx::Error> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .delete("id", ids)
        .await?;

    Ok(result)
}
