use sqlx::PgPool;

use crate::{
    dto::post_dto::PostDTO,
    repositories::posts::select_all_posts::select_all_posts,
};

pub async fn get_all_posts_service(
    pool: &PgPool,
) -> Result<Vec<PostDTO>, sqlx::Error> {
    let posts = select_all_posts(pool).await?;

    Ok(posts)
}
