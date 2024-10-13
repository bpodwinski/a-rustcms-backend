use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;

use crate::{
    dtos::{
        pagination_dto::PaginationDTO,
        user_dtos::{CreateUserDTO, DeleteUserIdsDTO, UserDTO},
    },
    models::users_models::UserModel,
    repositories::users_repository::{
        count_users, delete_user_by_id, insert_user, select_user_by_email,
        select_user_by_id, select_users, update_user,
    },
};

use super::calculate_pagination;

pub async fn create_user_service(
    pool: &PgPool,
    user_dto: CreateUserDTO,
) -> Result<UserDTO> {
    let mut user_model: UserModel = user_dto.try_into()?;

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(user_model.password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
        .to_string();

    user_model.password = password_hash;

    let create_user_model = insert_user(pool, user_model).await?;
    let result = UserDTO::from(create_user_model);

    Ok(result)
}

pub async fn update_user_service(
    pool: &PgPool,
    id: i32,
    user_dto: CreateUserDTO,
) -> Result<UserDTO> {
    let mut user_model: UserModel = user_dto.try_into()?;

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(user_model.password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
        .to_string();

    user_model.password = password_hash;

    let create_user_model = update_user(pool, id, user_model).await?;
    let result = UserDTO::from(create_user_model);

    Ok(result)
}

pub async fn get_all_users_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<PaginationDTO<UserDTO>> {
    let total_items = count_users(pool).await?;
    let pagination = calculate_pagination(total_items, page, limit);

    let user_model: Vec<UserModel> =
        select_users(pool, limit, pagination.offset, sort_column, sort_order)
            .await?;

    let user_dto: Vec<UserDTO> =
        user_model.into_iter().map(UserDTO::from).collect();

    Ok(PaginationDTO {
        current_page: pagination.current_page,
        total_pages: pagination.total_pages,
        total_items: pagination.total_items,
        data: user_dto,
    })
}

pub async fn get_user_by_id_service(pool: &PgPool, id: i32) -> Result<UserDTO> {
    let user_model = select_user_by_id(pool, id).await?;
    let result = UserDTO::from(user_model);
    Ok(result)
}

pub async fn get_user_by_email_service(
    pool: &PgPool,
    email: &str,
) -> Result<UserDTO> {
    let user_model = select_user_by_email(pool, email).await?;
    let result = UserDTO::from(user_model);
    Ok(result)
}

pub async fn delete_user_by_id_service(
    pool: &PgPool,
    delete_user_ids_dto: DeleteUserIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids = delete_user_by_id(pool, delete_user_ids_dto.ids).await?;
    Ok(deleted_ids)
}
