use anyhow::Result;
use sqlx::PgPool;

use crate::models::users_models::UserModel;

use super::{Bind, QueryBuilder};

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
