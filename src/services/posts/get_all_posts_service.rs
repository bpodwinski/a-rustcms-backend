use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::{
        pagination_dto::PaginationDTO,
        post_dto::{PostDTO, SortColumn, SortOrder},
    },
    repositories::posts::{
        count_total_posts_repository, select_all_posts_repository,
    },
};

pub async fn get_all_posts_service(
    pool: &PgPool,
    page: i64,
    limit: i64,
    sort_column: SortColumn,
    sort_order: SortOrder,
) -> Result<PaginationDTO<PostDTO>, sqlx::Error> {
    let total_items = count_total_posts_repository::select(pool).await?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

    let offset = (page - 1) * limit;
    let current_page = if page > total_pages {
        total_pages
    } else {
        page
    };

    let sort_column_str = match sort_column {
        SortColumn::Id => "p.id",
        SortColumn::Title => "p.title",
        SortColumn::AuthorId => "p.author_id",
        SortColumn::DatePublished => "p.date_published",
    };

    let sort_order_str = match sort_order {
        SortOrder::Asc => "ASC",
        SortOrder::Desc => "DESC",
    };

    let posts = select_all_posts_repository::select(
        pool,
        sort_column_str,
        sort_order_str,
        limit,
        offset,
    )
    .await?;

    Ok(PaginationDTO {
        current_page,
        total_pages,
        total_items,
        data: posts,
    })
}
