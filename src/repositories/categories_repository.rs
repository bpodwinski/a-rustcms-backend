use anyhow::Result;
use sqlx::PgPool;

use crate::models::categories_model::CategoryModel;

use super::{Bind, QueryBuilder};

/// Inserts a new category into the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `category_model` - The `CategoryModel` instance containing the category data to insert.
///
/// # Returns
///
/// * `Result<CategoryModel>` - The newly inserted `CategoryModel` record.
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

/// Updates an existing category in the database by its ID.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the category to update.
/// * `model` - The `CategoryModel` instance containing the updated category data.
///
/// # Returns
///
/// * `Result<CategoryModel>` - The updated `CategoryModel` record.
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

/// Retrieves all categories from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// * `Result<Vec<CategoryModel>>` - A vector containing the retrieved `CategoryModel` records.
pub async fn select_categories(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<Vec<CategoryModel>> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .limit(limit)
        .offset(offset)
        .sort_column(sort_column)
        .sort_order(sort_order)
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

/// Retrieves a category by its ID from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the category to retrieve.
///
/// # Returns
///
/// * `Result<CategoryModel>` - The `CategoryModel` record for the specified ID.
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

/// Deletes categories by their IDs from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `ids` - A vector containing the IDs of the categories to delete.
///
/// # Returns
///
/// * `Result<Vec<i32>>` - A vector containing the IDs of the deleted categories.
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

pub async fn count_categories(pool: &PgPool) -> Result<i64> {
    let result = QueryBuilder::<CategoryModel>::new(pool)
        .table("categories")
        .count()
        .await?;

    Ok(result)
}
