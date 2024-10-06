use anyhow::{Context, Result};
use sqlx::PgPool;

use crate::dtos::category_dto::{
    CategoryDTO, CreateCategoryDTO, DeleteCategoryIdsDTO,
};
use crate::models::categories_model::CategoryModel;
use crate::repositories::categories_repository::{
    delete_category_by_id, insert_category, select_categories,
    select_category_by_id, update_category,
};

pub async fn create_category_service(
    pool: &PgPool,
    create_category_dto: CreateCategoryDTO,
) -> Result<CategoryModel> {
    let category_model: CategoryModel = create_category_dto.try_into()?;

    let result = insert_category(pool, category_model).await?;
    Ok(result)
}

pub async fn get_all_categories_service(
    pool: &PgPool,
) -> Result<Vec<CategoryDTO>> {
    let category_models: Vec<CategoryModel> = select_categories(pool)
        .await
        .context("Failed to fetch categories from the database")?;

    let category_dtos: Vec<CategoryDTO> =
        category_models.into_iter().map(CategoryDTO::from).collect();

    Ok(category_dtos)
}

pub async fn get_category_by_id_service(
    pool: &PgPool,
    id: i32,
) -> Result<CategoryDTO> {
    let category_model: CategoryModel = select_category_by_id(pool, id).await?;

    let category_dto = CategoryDTO::from(category_model);

    Ok(category_dto)
}

pub async fn update_category_service(
    pool: &PgPool,
    id: i32,
    update_category_dto: CategoryDTO,
) -> Result<CategoryModel> {
    let mut category_model: CategoryModel = update_category_dto.try_into()?;
    category_model.id = Some(id);

    let result = update_category(pool, id, category_model).await?;
    Ok(result)
}

pub async fn delete_category_service(
    pool: &PgPool,
    delete_category_ids_dto: DeleteCategoryIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids =
        delete_category_by_id(pool, delete_category_ids_dto.ids).await?;
    Ok(deleted_ids)
}
