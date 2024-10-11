use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags_model::TagModel;

use super::{Bind, QueryBuilder};

/// Inserts a new tag into the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `tag_model` - The `TagModel` instance containing the tag data to insert.
///
/// # Returns
///
/// * `Result<TagModel>` - The newly inserted `TagModel` record.
pub async fn insert_tag(
    pool: &PgPool,
    tag_model: TagModel,
) -> Result<TagModel> {
    let result = QueryBuilder::<TagModel>::new(&pool)
        .table("tags")
        .fields(&["name", "slug", "description"])
        .values(vec![
            Bind::Text(tag_model.name),
            tag_model.slug.map_or(Bind::Null, Bind::Text),
            tag_model.description.map_or(Bind::Null, Bind::Text),
        ])
        .insert()
        .await?;

    Ok(result)
}

/// Updates an existing tag in the database by its ID.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the tag to update.
/// * `tag_model` - The `TagModel` instance containing the updated tag data.
///
/// # Returns
///
/// * `Result<TagModel>` - The updated `TagModel` record.
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
            tag_model.slug.map_or(Bind::Null, Bind::Text),
            tag_model.description.map_or(Bind::Null, Bind::Text),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

/// Retrieves all tags from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// * `Result<Vec<TagModel>>` - A vector containing all `TagModel` records.
pub async fn select_tags(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<Vec<TagModel>> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .limit(limit)
        .offset(offset)
        .sort_column(sort_column)
        .sort_order(sort_order)
        .fields(&["id", "name", "slug", "description", "date_created"])
        .select(None, None)
        .await?;

    Ok(result)
}

/// Retrieves a tag by its ID from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the tag to retrieve.
///
/// # Returns
///
/// * `Result<TagModel>` - The `TagModel` record for the specified ID.
pub async fn select_tag_by_id(pool: &PgPool, id: i32) -> Result<TagModel> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .fields(&["id", "name", "slug", "description", "date_created"])
        .select_one(Some("id"), Some(&Bind::Int(id)))
        .await?;

    Ok(result)
}

/// Deletes tags by their IDs from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `ids` - A vector containing the IDs of the tags to delete.
///
/// # Returns
///
/// * `Result<Vec<i32>>` - A vector containing the IDs of the deleted tags.
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

/// Counts the total number of tags in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// * `Result<i64>` - The total number of tags.
pub async fn count_tags(pool: &PgPool) -> Result<i64> {
    let result = QueryBuilder::<TagModel>::new(pool)
        .table("tags")
        .count()
        .await?;

    Ok(result)
}
