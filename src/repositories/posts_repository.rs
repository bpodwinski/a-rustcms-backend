use anyhow::Result;
use sqlx::PgPool;

use crate::models::posts_model::PostModel;

use super::{Bind, QueryBuilder};

/// Inserts a new post into the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `model` - The `PostModel` instance containing the post data to insert.
///
/// # Returns
///
/// * `Result<PostModel>` - The newly inserted `PostModel` record.
pub async fn insert_post(
    pool: &PgPool,
    post_model: PostModel,
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
            Bind::Text(post_model.title),
            Bind::Text(post_model.content),
            post_model.slug.map_or(Bind::Null, Bind::Text),
            Bind::Int(post_model.author_id),
        ])
        .insert()
        .await?;

    Ok(result)
}

/// Updates an existing post in the database by its ID.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the post to update.
/// * `model` - The `PostModel` instance containing the updated post data.
///
/// # Returns
///
/// * `Result<PostModel>` - The updated `PostModel` record.
pub async fn update_post(
    pool: &PgPool,
    id: i32,
    post_model: PostModel,
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
            Bind::Text(post_model.title),
            Bind::Text(post_model.content),
            post_model.slug.map_or(Bind::Null, Bind::Text),
            Bind::Int(post_model.author_id),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

/// Retrieves a paginated list of posts from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `limit` - The maximum number of posts to retrieve.
/// * `offset` - The number of posts to skip before starting to retrieve the records.
///
/// # Returns
///
/// * `Result<Vec<PostModel>>` - A vector containing the retrieved `PostModel` records.
pub async fn select_posts(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<Vec<PostModel>> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .limit(limit)
        .offset(offset)
        .sort_column(sort_column)
        .sort_order(sort_order)
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

/// Retrieves a post by its ID from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the post to retrieve.
///
/// # Returns
///
/// * `Result<PostModel>` - The `PostModel` record for the specified ID.
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

/// Deletes posts by their IDs from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `ids` - A vector containing the IDs of the posts to delete.
///
/// # Returns
///
/// * `Result<Vec<i32>>` - A vector containing the IDs of the deleted posts.
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

/// Counts the total number of posts in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// * `Result<i64>` - The total number of posts.
pub async fn count_posts(pool: &PgPool) -> Result<i64> {
    let result = QueryBuilder::<PostModel>::new(pool)
        .table("posts")
        .count()
        .await?;

    Ok(result)
}
