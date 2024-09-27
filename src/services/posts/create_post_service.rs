use anyhow::Result;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    dtos::{post_dto::PostDTO, posts_categories_dto::PostsCategoriesDTO},
    models::posts::posts_table_model::PostModel,
    repositories::posts_repository::insert_post,
};

pub async fn create_post_service(
    pool: &PgPool,
    post: PostModel,
    categories_ids: Vec<i32>,
) -> Result<PostDTO, sqlx::Error> {
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // Insert the post into database and retrieve post ID
    let post_id = insert_post(&mut transaction, &post).await?;

    let mut categories = Vec::new();

    // Insert associated categories
    for id in categories_ids {
        let post_category = PostsCategoriesDTO {
            id: None,
            post_id,
            category_id: id,
            date_created: None,
        };

        insert_posts_categories_repository::insert(
            &mut transaction,
            post_category.post_id,
            post_category.category_id,
        )
        .await?;

        categories.push(id);
    }

    // Commit transaction to make changes permanent
    transaction.commit().await?;

    // Retrieve post into database
    let post = select_posts(pool, post_id).await?;

    Ok(post)
}
