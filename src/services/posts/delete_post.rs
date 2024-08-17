use crate::repositories::posts::delete_post_by_id::delete_post_by_id;
use sqlx::PgPool;

pub async fn delete_post_service(
    pool: &PgPool,
    post_id: i32,
) -> Result<u64, sqlx::Error> {
    delete_post_by_id(pool, post_id).await
}
