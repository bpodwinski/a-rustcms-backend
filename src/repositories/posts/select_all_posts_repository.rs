use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::post_dto::PostDTO, models::posts::posts_type_model::PostsStatus,
};

pub async fn select(
    pool: &PgPool,
    sort_column: &str,
    sort_order: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostDTO>, sqlx::Error> {
    let rows = sqlx::query_file!(
        "src/repositories/posts/select_all_posts.sql",
        sort_column,
        sort_order,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await?;

    let posts = rows
        .into_iter()
        .map(|row| PostDTO {
            id: Some(row.id),
            title: row.title,
            content: row.content,
            slug: row.slug,
            author_id: row.author_id,
            status: PostsStatus::from(row.status),
            date_published: row.date_published,
            date_created: Some(row.date_created),
            categories: row.categories,
        })
        .collect();

    Ok(posts)
}
