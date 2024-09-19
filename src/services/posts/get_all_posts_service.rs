use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::{pagination_dto::PaginationDTO, post_dto::PostDTO},
    repositories::posts::{
        count_total_posts_repository, select_all_posts_repository,
    },
};

pub async fn get_all_posts_service(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<PaginationDTO<PostDTO>, sqlx::Error> {
    let total_items = count_total_posts_repository::select(pool).await?;

    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;
    let current_page = (offset / limit) + 1;

    let posts =
        select_all_posts_repository::select(pool, limit, offset).await?;

    Ok(PaginationDTO {
        current_page,
        total_pages,
        total_items,
        data: posts,
    })
}
