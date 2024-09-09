use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::models::posts::posts_table_model::Post;
use crate::models::posts::posts_type_model::PostsStatus;
use crate::repositories::posts::{
    select_post_by_id::select_post_by_id, update_post_by_id::update_post_by_id,
};

pub async fn update_post_service(
    pool: &PgPool,
    post_id: i32,
    title: String,
    content: String,
    status: PostsStatus,
    date_published: Option<NaiveDateTime>,
) -> Result<Post, sqlx::Error> {
    let _ = update_post_by_id(
        pool,
        post_id,
        title.clone(),
        content.clone(),
        status.clone(),
        date_published,
    )
    .await?;
    let updated_post = select_post_by_id(pool, post_id).await?;
    Ok(updated_post)
}
