use anyhow::Result;
use sqlx::PgPool;

use crate::models::posts_model::PostModel;

use super::{Bind, QueryBuilder};

pub async fn insert_post(pool: &PgPool, model: PostModel) -> Result<PostModel> {
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
) -> Result<PostModel> {
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

pub async fn select_posts(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostModel>> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .limit(limit)
        .offset(offset)
        .fields(&[
            "id",
            "title",
            "content",
            "slug",
            "author_id",
            "status",
            "date_published",
            "date_created",
            "categories",
        ])
        .select(None, None)
        .await?;

    Ok(result)
}

pub async fn select_post_by_id(pool: &PgPool, id: i32) -> Result<PostModel> {
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

pub async fn delete_post_by_id(
    pool: &PgPool,
    ids: Vec<i32>,
) -> Result<Vec<i32>> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .delete("id", ids)
        .await?;

    Ok(result)
}

pub async fn count_posts(pool: &PgPool) -> Result<i64> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .count()
        .await?;

    Ok(result)
}
