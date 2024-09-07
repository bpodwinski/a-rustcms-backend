use sqlx::{Acquire, Postgres, Transaction};

use crate::{
    dtos::post_dto::PostId,
    models::posts::{posts_table_model::Post, posts_type_model::PostsStatus},
};

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    post: &Post,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_file_as!(
        PostId,
        "src/repositories/posts/insert_post.sql",
        post.title,
        post.content,
        post.slug,
        post.author_id,
        post.status.clone() as PostsStatus,
        post.date_published
    )
    .fetch_one(&mut *tx.acquire().await?)
    .await?;

    Ok(result.id)
}
