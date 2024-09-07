use sqlx::PgPool;

use crate::{
    dtos::post_dto::PostDTO, models::posts::posts_type_model::PostsStatus,
};

pub async fn select(
    pool: &PgPool,
    post_id: i32,
) -> Result<PostDTO, sqlx::Error> {
    // Execute SQL query to fetch the post with the specified ID
    let row = sqlx::query_file!(
        "src/repositories/posts/select_post_by_id.sql",
        post_id
    )
    .fetch_one(pool)
    .await?;

    // Map the result to a PostDTO
    let post = PostDTO {
        id: Some(row.id),
        title: row.title,
        content: row.content,
        slug: row.slug,
        author_id: row.author_id,
        status: PostsStatus::from(row.status),
        date_published: row.date_published,
        date_created: Some(row.date_created),
        categories: row.categories,
    };

    Ok(post)
}
