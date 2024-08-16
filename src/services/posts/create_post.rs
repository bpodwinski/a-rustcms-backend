use sqlx::PgPool;

use crate::{
    models::posts::posts_table_model::Post,
    repositories::posts::{
        insert_post::insert_post, select_post_by_id::select_post_by_id,
    },
};

pub async fn create_post_service(
    pool: PgPool,
    post: Post,
) -> Result<Post, sqlx::Error> {
    let post_id = insert_post(&pool, post).await?;
    let post = select_post_by_id(&pool, post_id).await?;
    Ok(post)
}
