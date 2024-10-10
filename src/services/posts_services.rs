use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::pagination_dto::PaginationDTO;
use crate::dtos::post_dto::{
    CreatePostDTO, DeletePostIdsDTO, PostDTO, SortColumn, SortOrder,
};
use crate::models::posts_model::PostModel;
use crate::repositories::posts_repository::{
    count_posts, delete_post_by_id, insert_post, select_post_by_id,
    select_posts, update_post,
};

/// Service pour insérer un post dans la base de données.
pub async fn create_post_service(
    pool: &PgPool,
    create_post_dto: CreatePostDTO,
) -> Result<PostModel> {
    let post_model: PostModel = create_post_dto.try_into()?;

    let result = insert_post(pool, post_model).await?;
    Ok(result)
}

/// Service pour récupérer tous les posts dans la base de données.
pub async fn get_all_posts_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: &str,
    sort_order: &str,
) -> Result<PaginationDTO<PostDTO>> {
    let total_items = count_posts(pool).await?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

    let offset = (page - 1) * limit;
    let current_page = if page > total_pages {
        total_pages
    } else {
        page
    };

    let posts_model =
        select_posts(pool, limit, offset, sort_column, sort_order).await?;

    let posts_dto: Vec<PostDTO> =
        posts_model.into_iter().map(PostDTO::from).collect();

    Ok(PaginationDTO {
        current_page,
        total_pages,
        total_items,
        data: posts_dto,
    })
}

/// Service pour récupérer un post par son ID dans la base de données.
pub async fn get_post_by_id_service(pool: &PgPool, id: i32) -> Result<PostDTO> {
    let post_model: PostModel = select_post_by_id(pool, id).await?;

    let post_dto = PostDTO::from(post_model);

    Ok(post_dto)
}

/// Service pour mettre à jour un post par son ID dans la base de données.
pub async fn update_post_service(
    pool: &PgPool,
    id: i32,
    update_post_dto: CreatePostDTO,
) -> Result<PostModel> {
    let mut post_model: PostModel = update_post_dto.try_into()?;
    post_model.id = Some(id);

    let result = update_post(pool, id, post_model).await?;
    Ok(result)
}

/// Service pour supprimer des posts par ID dans la base de données.
pub async fn delete_post_service(
    pool: &PgPool,
    delete_post_ids_dto: DeletePostIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids = delete_post_by_id(pool, delete_post_ids_dto.ids).await?;
    Ok(deleted_ids)
}
