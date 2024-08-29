use crate::dto::{category_dto::CategoryInfo, post_dto::PostWithCategories};
use crate::models::posts::posts_categories_table_model::PostsCategories;
use crate::models::posts::posts_table_model::Post;
use crate::repositories::posts::insert_post::insert_post;
use crate::repositories::posts_categories::insert_posts_categories::insert_posts_categories;
use sqlx::{PgPool, Postgres, Transaction};

pub async fn create_post_service(
    pool: &PgPool,
    post: Post,
    categories_ids: Vec<i32>,
) -> Result<PostWithCategories, sqlx::Error> {
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // Insérer le post et récupérer l'ID généré
    let post_id = insert_post(&mut transaction, &post).await?;

    let mut categories = Vec::new();

    // Insérer les catégories associées
    for category_id in categories_ids {
        let post_category = PostsCategories {
            id: None,
            post_id,
            category_id: category_id,
            date_created: None,
        };

        let _ = insert_posts_categories(
            &mut transaction,
            post_category.post_id,
            post_category.category_id,
        )
        .await?;

        categories.push(CategoryInfo { category_id });
    }

    transaction.commit().await?;

    let post_with_categories = PostWithCategories {
        id: post_id,
        title: post.title,
        content: post.content,
        author_id: post.author_id,
        status: post.status,
        date_published: post.date_published,
        date_created: post.date_created,
        categories,
    };

    Ok(post_with_categories)
}
