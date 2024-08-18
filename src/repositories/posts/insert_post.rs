use sqlx::{Acquire, Postgres, Transaction};

use crate::models::posts::{posts_table_model::Post, posts_type_model::Status};

#[derive(sqlx::FromRow)]
struct PostId {
    id: i32,
}

pub async fn insert_post(
    tx: &mut Transaction<'_, Postgres>,
    post: &Post,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_file_as!(
        PostId,
        "src/repositories/posts/insert_post.sql",
        post.title,
        post.content,
        post.author_id,
        post.status.clone() as Status,
        post.date_published
    )
    .fetch_one(&mut *tx.acquire().await?)
    .await?;

    Ok(result.id)
}
