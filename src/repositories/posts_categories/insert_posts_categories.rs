use sqlx::{Acquire, Postgres, Transaction};

use crate::models::posts::posts_categories_table_model::PostsCategories;

pub async fn insert_posts_categories(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i32,
    category_id: i32,
) -> Result<PostsCategories, sqlx::Error> {
    let result = sqlx::query_file_as!(
        PostsCategories,
        "src/repositories/posts_categories/insert_posts_categories.sql",
        post_id,
        category_id
    )
    .fetch_one(&mut *tx.acquire().await?)
    .await?;

    Ok(result)
}
