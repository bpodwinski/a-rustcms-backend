use sqlx::PgPool;

use crate::dtos::posts_categories_dto::PostsCategoriesDTO;

pub async fn delete(
    pool: &PgPool,
    post_id: i32,
) -> Result<Vec<PostsCategoriesDTO>, sqlx::Error> {
    let rows = sqlx::query_file_as!(
        PostsCategoriesDTO,
        "src/repositories/posts_categories/delete_by_post_id.sql",
        post_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
