use anyhow::Result;
use sqlx::PgPool;

use crate::models::users_models::UserModel;

use super::{Bind, QueryBuilder};

/// Inserts a user into the database.
///
/// # Arguments
/// * `pool` - Reference to the PgPool connection pool.
/// * `user_model` - The user model to be inserted.
///
/// # Returns
/// A `Result` containing the inserted `UserModel` or an error.
pub async fn insert_user(
    pool: &PgPool,
    user_model: UserModel,
) -> Result<UserModel> {
    let result = QueryBuilder::<UserModel>::new(&pool)
        .table("users")
        .fields(&[
            "username",
            "password",
            "email",
            "firstname",
            "lastname",
            "url",
            "active",
        ])
        .values(vec![
            Bind::Text(user_model.username),
            Bind::Text(user_model.password),
            Bind::Text(user_model.email),
            Bind::Text(user_model.firstname),
            Bind::Text(user_model.lastname),
            user_model.url.map_or(Bind::Null, Bind::Text),
            Bind::Bool(true),
        ])
        .insert()
        .await?;

    Ok(result)
}

/// Updates a user in the database.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
/// * `id` - The ID of the user to update.
/// * `user_model` - The user model with the new data.
///
/// # Returns
/// A `Result` containing the updated user model or an error.
pub async fn update_user(
    pool: &PgPool,
    id: i32,
    user_model: UserModel,
) -> Result<UserModel> {
    let result = QueryBuilder::<UserModel>::new(&pool)
        .table("users")
        .fields(&[
            "username",
            "password",
            "email",
            "firstname",
            "lastname",
            "url",
            "active",
        ])
        .values(vec![
            Bind::Text(user_model.username),
            Bind::Text(user_model.password),
            Bind::Text(user_model.email),
            Bind::Text(user_model.firstname),
            Bind::Text(user_model.lastname),
            user_model.url.map_or(Bind::Null, Bind::Text),
            Bind::Bool(true),
        ])
        .update("id", Bind::Int(id))
        .await?;

    Ok(result)
}

/// Selects a list of users with pagination and sorting options.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
/// * `limit` - The number of users to return.
/// * `offset` - The pagination offset.
/// * `sort_column` - The column to sort by.
/// * `sort_order` - The sort order (ASC or DESC).
///
/// # Returns
/// A `Result` containing a vector of `UserModel` or an error.
pub async fn select_users(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<Vec<UserModel>> {
    let result = QueryBuilder::<UserModel>::new(pool)
        .table("users")
        .limit(limit)
        .offset(offset)
        .sort_column(sort_column)
        .sort_order(sort_order)
        .fields(&[
            "id",
            "username",
            "password",
            "email",
            "firstname",
            "lastname",
            "url",
            "active",
            "date_created",
        ])
        .select(None, None)
        .await?;

    Ok(result)
}

/// Selects a user by ID.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
/// * `id` - The ID of the user to retrieve.
///
/// # Returns
/// A `Result` containing a `UserModel` or an error.
pub async fn select_user_by_id(pool: &PgPool, id: i32) -> Result<UserModel> {
    let result = QueryBuilder::<UserModel>::new(&pool)
        .table("users")
        .fields(&[
            "id",
            "username",
            "password",
            "email",
            "firstname",
            "lastname",
            "url",
            "active",
            "date_created",
        ])
        .select_one("id", Bind::Int(id))
        .await?;

    Ok(result)
}

/// Selects a user by email address.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
/// * `email` - The email of the user to retrieve.
///
/// # Returns
/// A `Result` containing a `UserModel` or an error.
pub async fn select_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<UserModel> {
    let result = QueryBuilder::<UserModel>::new(&pool)
        .table("users")
        .fields(&[
            "id",
            "username",
            "password",
            "email",
            "firstname",
            "lastname",
            "url",
            "active",
            "date_created",
        ])
        .select_one("email", Bind::Text(email.to_string()))
        .await?;

    Ok(result)
}

/// Deletes one or more users by their IDs.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
/// * `ids` - A vector containing the IDs of the users to delete.
///
/// # Returns
/// A `Result` containing a vector of deleted IDs or an error.
pub async fn delete_user_by_id(
    pool: &PgPool,
    ids: Vec<i32>,
) -> Result<Vec<i32>> {
    let result = QueryBuilder::<UserModel>::new(pool)
        .table("users")
        .delete("id", ids)
        .await?;

    Ok(result)
}

/// Counts the total number of users.
///
/// # Arguments
/// * `pool` - Reference to the PgPool pool.
///
/// # Returns
/// A `Result` containing the total user count or an error.
pub async fn count_users(pool: &PgPool) -> Result<i64> {
    let result = QueryBuilder::<UserModel>::new(pool)
        .table("users")
        .count()
        .await?;

    Ok(result)
}
