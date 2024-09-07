use sqlx::{Acquire, Postgres, Transaction};

use crate::dtos::posts_categories_dto::PostsCategoriesDTO;

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i32,
    category_id: i32,
) -> Result<PostsCategoriesDTO, sqlx::Error> {
    let result = sqlx::query_file_as!(
        PostsCategoriesDTO,
        "src/repositories/posts_categories/insert_posts_categories.sql",
        post_id,
        category_id
    )
    .fetch_one(&mut *tx.acquire().await?)
    .await?;

    Ok(result)
}
