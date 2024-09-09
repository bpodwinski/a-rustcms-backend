use anyhow::Result;
use sqlx::PgPool;

use crate::models::posts::posts_type_model::PostsStatus;

pub async fn update(
    pool: &PgPool,
    post_id: i32,
    title: String,
    content: String,
    slug: String,
    status: PostsStatus,
    date_published: Option<chrono::NaiveDateTime>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "src/repositories/posts/update_post_by_id.sql",
        post_id,
        title,
        content,
        slug,
        status as PostsStatus,
        date_published
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
