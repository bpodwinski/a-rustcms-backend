use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::category_dto::{
    CategoryDTO, CreateCategoryDTO, DeleteCategoryIdsDTO,
};
use crate::dtos::pagination_dto::PaginationDTO;
use crate::models::categories_model::CategoryModel;
use crate::repositories::categories_repository::{
    count_categories, delete_category_by_id, insert_category,
    select_categories, select_category_by_id, update_category,
};

use super::calculate_pagination;

/// Service to create a new category.
///
/// # Arguments
///
/// * `pool` - A reference to the Postgres connection pool.
/// * `create_category_dto` - DTO object containing the category data to be created.
///
/// # Returns
///
/// Returns a `CategoryDTO` representing the newly created category.
pub async fn create_category_service(
    pool: &PgPool,
    create_category_dto: CreateCategoryDTO,
) -> Result<CategoryDTO> {
    let category_model: CategoryModel = create_category_dto.try_into()?;

    let create_category_model = insert_category(pool, category_model).await?;
    let result = CategoryDTO::from(create_category_model);
    Ok(result)
}

/// Service to update an existing category by its ID.
///
/// # Arguments
///
/// * `pool` - A reference to the Postgres connection pool.
/// * `id` - The ID of the category to update.
/// * `category_dto` - DTO object containing the updated category data.
///
/// # Returns
///
/// Returns a `CategoryDTO` representing the updated category.
pub async fn update_category_service(
    pool: &PgPool,
    id: i32,
    category_dto: CreateCategoryDTO,
) -> Result<CategoryDTO> {
    let mut category_model: CategoryModel = category_dto.try_into()?;
    category_model.id = Some(id);

    let update_category_model =
        update_category(pool, id, category_model).await?;
    let result = CategoryDTO::from(update_category_model);
    Ok(result)
}

/// Service to retrieve all categories with pagination, sorting, and ordering options.
///
/// # Arguments
///
/// * `pool` - A reference to the Postgres connection pool.
/// * `page` - The current page number for pagination.
/// * `limit` - The number of items per page.
/// * `sort_column` - The column name to sort the results by.
/// * `sort_order` - The order of sorting (e.g., "asc" for ascending, "desc" for descending).
///
/// # Returns
///
/// Returns a `PaginationDTO` containing paginated category data.
pub async fn get_all_categories_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<PaginationDTO<CategoryDTO>> {
    let total_items = count_categories(pool).await?;
    let pagination = calculate_pagination(total_items, page, limit);

    let category_model: Vec<CategoryModel> = select_categories(
        pool,
        limit,
        pagination.offset,
        sort_column,
        sort_order,
    )
    .await?;

    let category_dto: Vec<CategoryDTO> =
        category_model.into_iter().map(CategoryDTO::from).collect();

    Ok(PaginationDTO {
        current_page: pagination.current_page,
        total_pages: pagination.total_pages,
        total_items: pagination.total_items,
        data: category_dto,
    })
}

/// Service to retrieve a category by its ID.
///
/// # Arguments
///
/// * `pool` - A reference to the Postgres connection pool.
/// * `id` - The ID of the category to retrieve.
///
/// # Returns
///
/// Returns a `CategoryDTO` representing the retrieved category.
pub async fn get_category_by_id_service(
    pool: &PgPool,
    id: i32,
) -> Result<CategoryDTO> {
    let category_model: CategoryModel = select_category_by_id(pool, id).await?;
    let result = CategoryDTO::from(category_model);
    Ok(result)
}

/// Service to delete categories by a list of IDs.
///
/// # Arguments
///
/// * `pool` - A reference to the Postgres connection pool.
/// * `delete_category_ids_dto` - DTO containing the list of category IDs to delete.
///
/// # Returns
///
/// Returns a `Vec<i32>` containing the IDs of the deleted categories.
pub async fn delete_category_service(
    pool: &PgPool,
    delete_category_ids_dto: DeleteCategoryIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids =
        delete_category_by_id(pool, delete_category_ids_dto.ids).await?;
    Ok(deleted_ids)
}
