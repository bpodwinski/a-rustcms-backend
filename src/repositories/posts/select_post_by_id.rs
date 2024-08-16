use sqlx::PgPool;

use crate::models::posts::{posts_table_model::Post, posts_type_model::Status};

#[derive(sqlx::FromRow)]
struct InsertPostResult {
    id: i32,
}

pub async fn select_post_by_id(
    pool: &PgPool,
    post_id: i32,
) -> Result<Post, sqlx::Error> {
    sqlx::query_file_as!(
        Post,
        "src/repositories/posts/select_post_by_id.sql",
        post_id
    )
    .fetch_one(pool)
    .await
}
