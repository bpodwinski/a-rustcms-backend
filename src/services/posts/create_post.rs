use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    models::posts::posts_categories_table_model::PostsCategories,
    models::posts::posts_table_model::Post,
    repositories::posts::insert_post::insert_post,
    repositories::posts_categories::insert_posts_categories::insert_posts_categories,
};

pub async fn create_post_service(
    pool: &PgPool,
    post: Post,
    categories_ids: Vec<i32>,
) -> Result<(Post, Vec<PostsCategories>), sqlx::Error> {
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    let post_id = insert_post(&mut transaction, &post).await?;

    let mut posts_categories = Vec::new();

    for category_id in categories_ids {
        let post_category = PostsCategories {
            id: None,
            post_id,
            categories_id: category_id,
            date_created: None,
        };

        let _ = insert_posts_categories(
            &mut transaction,
            post_category.post_id,
            post_category.categories_id,
        )
        .await?;

        posts_categories.push(post_category);
    }

    transaction.commit().await?;

    Ok((
        Post {
            id: Some(post_id),
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            status: post.status,
            date_published: post.date_published,
            date_created: post.date_created,
        },
        posts_categories,
    ))
}
