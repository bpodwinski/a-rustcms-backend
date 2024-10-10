use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::pagination_dto::PaginationDTO;
use crate::dtos::post_dto::{CreatePostDTO, DeletePostIdsDTO, PostDTO};
use crate::models::posts_model::PostModel;
use crate::repositories::posts_repository::{
    count_posts, delete_post_by_id, insert_post, select_post_by_id,
    select_posts, update_post,
};

use super::calculate_pagination;

/// Service to insert a post into the database.
///
/// # Arguments
///
/// * `pool` - Reference to the database connection pool.
/// * `create_post_dto` - Data Transfer Object containing the details for the new post.
///
/// # Returns
///
/// Returns a `PostDTO` object containing the inserted post details.
pub async fn create_post_service(
    pool: &PgPool,
    create_post_dto: CreatePostDTO,
) -> Result<PostDTO> {
    let post_model: PostModel = create_post_dto.try_into()?;

    let create_post_model = insert_post(pool, post_model).await?;
    let result = PostDTO::from(create_post_model);
    Ok(result)
}

/// Service to update a post by its ID in the database.
///
/// # Arguments
///
/// * `pool` - Reference to the database connection pool.
/// * `id` - The ID of the post to be updated.
/// * `update_post_dto` - Data Transfer Object containing the updated details of the post.
///
/// # Returns
///
/// Returns a `PostDTO` object containing the updated post details.
pub async fn update_post_service(
    pool: &PgPool,
    id: i32,
    update_post_dto: CreatePostDTO,
) -> Result<PostDTO> {
    let mut post_model: PostModel = update_post_dto.try_into()?;
    post_model.id = Some(id);

    let update_post_model = update_post(pool, id, post_model).await?;
    let result = PostDTO::from(update_post_model);
    Ok(result)
}

/// Service to retrieve all posts from the database with pagination and sorting.
///
/// # Arguments
///
/// * `pool` - Reference to the database connection pool.
/// * `page` - The current page number for pagination.
/// * `limit` - The number of posts per page.
/// * `sort_column` - The column name to sort by.
/// * `sort_order` - The order of sorting (either "asc" for ascending or "desc" for descending).
///
/// # Returns
///
/// Returns a `PaginationDTO<PostDTO>` object containing the paginated list of posts and pagination information.
pub async fn get_all_posts_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<PaginationDTO<PostDTO>> {
    let total_items = count_posts(pool).await?;
    let pagination = calculate_pagination(total_items, page, limit);

    let posts_model =
        select_posts(pool, limit, pagination.offset, sort_column, sort_order)
            .await?;

    let posts_dto: Vec<PostDTO> =
        posts_model.into_iter().map(PostDTO::from).collect();

    Ok(PaginationDTO {
        current_page: pagination.current_page,
        total_pages: pagination.total_pages,
        total_items: pagination.total_items,
        data: posts_dto,
    })
}

/// Service to retrieve a post by its ID from the database.
///
/// # Arguments
///
/// * `pool` - Reference to the database connection pool.
/// * `id` - The ID of the post to retrieve.
///
/// # Returns
///
/// Returns a `PostDTO` object containing the details of the post.
pub async fn get_post_by_id_service(pool: &PgPool, id: i32) -> Result<PostDTO> {
    let post_model: PostModel = select_post_by_id(pool, id).await?;
    let post_dto = PostDTO::from(post_model);
    Ok(post_dto)
}

/// Service to delete posts by their IDs in the database.
///
/// # Arguments
///
/// * `pool` - Reference to the database connection pool.
/// * `delete_post_ids_dto` - Data Transfer Object containing the list of post IDs to delete.
///
/// # Returns
///
/// Returns a vector of deleted post IDs.
pub async fn delete_post_service(
    pool: &PgPool,
    delete_post_ids_dto: DeletePostIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids = delete_post_by_id(pool, delete_post_ids_dto.ids).await?;
    Ok(deleted_ids)
}
