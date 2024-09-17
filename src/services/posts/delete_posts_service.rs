use anyhow::Result;
use sqlx::PgPool;

use crate::repositories::{
    posts::delete_posts_by_ids_repository,
    posts_categories::delete_by_post_id_repository,
};

pub async fn delete_posts_service(
    pool: &PgPool,
    posts_ids: Vec<i32>,
) -> Result<Vec<i32>, sqlx::Error> {
    // Delete associated categories first
    for post_id in &posts_ids {
        delete_by_post_id_repository::delete(pool, *post_id).await?;
    }

    // Delete posts
    let deleted_ids =
        delete_posts_by_ids_repository::delete(pool, posts_ids).await?;

    // Return the number of affected rows from posts deletion
    Ok(deleted_ids)
}
