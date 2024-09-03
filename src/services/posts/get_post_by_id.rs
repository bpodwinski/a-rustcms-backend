use sqlx::PgPool;

use crate::{ dto::post_dto::PostDTO, repositories::posts::select_post_by_id::select_post_by_id };

pub async fn get_post_by_id_service(pool: &PgPool, post_id: i32) -> Result<PostDTO, sqlx::Error> {
    let post = select_post_by_id(pool, post_id).await?;
    let post_dto = PostDTO::from(post);

    Ok(post_dto)
}
