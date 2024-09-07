use crate::{
    dtos::{
        category_dto::CategoryId, post_dto::PostDTO,
        posts_categories_dto::PostsCategoriesDTO,
    },
    models::posts::posts_table_model::Post,
    repositories::{
        posts::{insert_post_repository, select_post_by_id_repository},
        posts_categories::insert_posts_categories_repository,
    },
};
use sqlx::{PgPool, Postgres, Transaction};

pub async fn create_post_service(
    pool: &PgPool,
    post: Post,
    categories_ids: Vec<i32>,
) -> Result<PostDTO, sqlx::Error> {
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // Insert the post into database and retrieve post ID
    let post_id =
        insert_post_repository::insert(&mut transaction, &post).await?;

    let mut categories = Vec::new();

    // Insert associated categories
    for category_id in categories_ids {
        let post_category = PostsCategoriesDTO {
            id: None,
            post_id,
            category_id: category_id,
            date_created: None,
        };

        insert_posts_categories_repository::insert(
            &mut transaction,
            post_category.post_id,
            post_category.category_id,
        )
        .await?;

        categories.push(CategoryId { id: category_id });
    }

    // Commit transaction to make changes permanent
    transaction.commit().await?;

    // Retrieve post into database
    let post = select_post_by_id_repository::select(pool, post_id).await?;

    Ok(post)
}
