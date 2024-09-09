use anyhow::Result;
use sqlx::PgPool;

use crate::repositories::{
    posts::delete_post_by_id_repository,
    posts_categories::delete_by_post_id_repository,
};

pub async fn delete_post_service(
    pool: &PgPool,
    post_id: i32,
) -> Result<u64, sqlx::Error> {
    // Delete associated categories first
    delete_by_post_id_repository::delete(pool, post_id).await?;

    // Delete the post itself
    let rows_affected =
        delete_post_by_id_repository::delete(pool, post_id).await?;

    // Return the number of affected rows from the post deletion
    Ok(rows_affected)
}
