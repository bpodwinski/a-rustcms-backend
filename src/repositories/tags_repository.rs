use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags_model::TagModel;

use super::{Bind, QueryBuilder};

pub async fn insert_tag(
    pool: &PgPool,
    tag_model: TagModel,
) -> Result<TagModel> {
    let result = QueryBuilder::<TagModel>::new(&pool)
        .table("tags")
        .fields(&["name", "slug", "description"])
        .values(vec![
            Bind::Text(tag_model.name),
            Bind::Text(tag_model.slug),
            tag_model.description.map_or(Bind::Null, Bind::Text),
        ])
        .insert()
        .await?;

    Ok(result)
}

pub async fn update_tag(
    pool: &PgPool,
    id: i32,
    tag_model: TagModel,
) -> Result<TagModel> {
    let result = QueryBuilder::<TagModel>::new(&pool)
        .table("tags")
        .fields(&["name", "slug", "description"])
        .values(vec![
            Bind::Text(tag_model.name),
            Bind::Text(tag_model.slug),
            tag_model.description.map_or(Bind::Null, Bind::Text),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

pub async fn select_tags(pool: &PgPool) -> Result<Vec<TagModel>> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .fields(&["id", "name", "slug", "description", "date_created"])
        .select(None, None)
        .await?;

    Ok(result)
}

pub async fn select_tag_by_id(pool: &PgPool, id: i32) -> Result<TagModel> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .fields(&["id", "name", "slug", "description", "date_created"])
        .select_one(Some("id"), Some(&Bind::Int(id)))
        .await?;

    Ok(result)
}

pub async fn delete_tag_by_id(
    pool: &PgPool,
    ids: Vec<i32>,
) -> Result<Vec<i32>> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .delete("id", ids)
        .await?;

    Ok(result)
}
