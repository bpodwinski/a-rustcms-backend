use sqlx::PgPool;

use crate::{
    models::posts::posts_table_model::Post,
    repositories::posts::select_all_posts::select_all_posts,
};

pub async fn get_all_posts_service(
    pool: &PgPool,
) -> Result<Vec<Post>, sqlx::Error> {
    let posts = select_all_posts(pool).await?;
    Ok(posts)
}
