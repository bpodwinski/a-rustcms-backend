use sqlx::PgPool;

use crate::models::posts::{posts_table_model::Post, posts_type_model::Status};

#[derive(sqlx::FromRow)]
struct InsertPostResult {
    id: i32,
}

pub async fn select_all_posts(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_file_as!(Post, "src/repositories/posts/select_all_posts.sql")
        .fetch_all(pool)
        .await
}
