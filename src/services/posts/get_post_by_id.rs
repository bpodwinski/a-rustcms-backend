use sqlx::PgPool;

use crate::{
    models::posts::posts_table_model::Post,
    repositories::posts::select_post_by_id::select_post_by_id,
};

pub async fn get_post_by_id_service(
    pool: &PgPool,
    post_id: i32,
) -> Result<Post, sqlx::Error> {
    select_post_by_id(pool, post_id).await
}
