use sqlx::PgPool;

use crate::models::posts::{posts_table_model::Post, posts_type_model::Status};

#[derive(sqlx::FromRow)]
struct InsertPostResult {
    id: i32,
}

pub async fn insert_post(
    pool: &PgPool,
    post: Post,
) -> Result<i32, sqlx::Error> {
    let response = sqlx::query_file_as!(
        InsertPostResult,
        "src/repositories/posts/insert_post.sql",
        post.title,
        post.content,
        post.author_id,
        post.status as Status,
        post.date_published
    )
    .fetch_one(pool)
    .await?;
    Ok(response.id)
}
