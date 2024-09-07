use sqlx::PgPool;

use crate::{
    dtos::post_dto::PostDTO, repositories::posts::select_all_posts_repository,
};

pub async fn get_all_posts_service(
    pool: &PgPool,
) -> Result<Vec<PostDTO>, sqlx::Error> {
    let posts = select_all_posts_repository::select(pool).await?;

    Ok(posts)
}
