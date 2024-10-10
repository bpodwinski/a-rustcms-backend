use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::pagination_dto::PaginationDTO;
use crate::dtos::tag_dto::{CreateTagDTO, DeleteTagIdsDTO, TagDTO};
use crate::models::tags_model::TagModel;
use crate::repositories::tags_repository::{
    count_tags, delete_tag_by_id, insert_tag, select_tag_by_id, select_tags,
    update_tag,
};

use super::calculate_pagination;

/// Service to insert a new tag into the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `create_tag_dto` - A `CreateTagDTO` object containing the tag data.
///
/// # Returns
///
/// * `Result<TagModel>` - The newly inserted `TagModel` record.
pub async fn create_tag_service(
    pool: &PgPool,
    tag_dto: CreateTagDTO,
) -> Result<TagDTO> {
    let tag_model: TagModel = tag_dto.try_into()?;

    let create_tag_model = insert_tag(pool, tag_model).await?;
    let result = TagDTO::from(create_tag_model);
    Ok(result)
}

/// Service to update an existing tag by its ID in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the tag to update.
/// * `update_tag_dto` - A `CreateTagDTO` object containing the updated tag data.
///
/// # Returns
///
/// * `Result<TagModel>` - The updated `TagModel` record.
pub async fn update_tag_service(
    pool: &PgPool,
    id: i32,
    tag_dto: CreateTagDTO,
) -> Result<TagDTO> {
    let mut tag_model: TagModel = tag_dto.try_into()?;
    tag_model.id = Some(id);

    let update_tag_model = update_tag(pool, id, tag_model).await?;
    let result = TagDTO::from(update_tag_model);
    Ok(result)
}

/// Service to retrieve all tags from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
///
/// * `Result<Vec<TagDTO>>` - A vector containing the `TagDTO` records.
pub async fn get_all_tags_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<PaginationDTO<TagDTO>> {
    let total_items = count_tags(pool).await?;
    let pagination = calculate_pagination(total_items, page, limit);

    let tags_model =
        select_tags(pool, limit, pagination.offset, sort_column, sort_order)
            .await?;

    let tags_dto: Vec<TagDTO> =
        tags_model.into_iter().map(TagDTO::from).collect();

    Ok(PaginationDTO {
        current_page: pagination.current_page,
        total_pages: pagination.total_pages,
        total_items: pagination.total_items,
        data: tags_dto,
    })
}

/// Service to retrieve a tag by its ID from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `id` - The ID of the tag to retrieve.
///
/// # Returns
///
/// * `Result<TagModel>` - The `TagModel` record for the specified ID.
pub async fn get_tag_by_id_service(pool: &PgPool, id: i32) -> Result<TagDTO> {
    let tag_model = select_tag_by_id(pool, id).await?;
    let result = TagDTO::from(tag_model);
    Ok(result)
}

/// Service to delete tags by their IDs in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `delete_tag_ids_dto` - A `DeleteTagIdsDTO` containing the list of tag IDs to delete.
///
/// # Returns
///
/// * `Result<Vec<i32>>` - A vector containing the IDs of the deleted tags.
pub async fn delete_tag_by_id_service(
    pool: &PgPool,
    delete_tag_ids_dto: DeleteTagIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids = delete_tag_by_id(pool, delete_tag_ids_dto.ids).await?;
    Ok(deleted_ids)
}
