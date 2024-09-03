use crate::dto::{category_dto::CategoryId, post_dto::PostDTO};
use crate::models::posts::posts_table_model::Post;
use crate::models::posts_categories::posts_categories_table_model::PostsCategories;
use crate::repositories::posts::insert_post::insert_post;
use crate::repositories::posts::select_post_by_id::select_post_by_id;
use crate::repositories::posts_categories::insert_posts_categories::insert_posts_categories;
use sqlx::{PgPool, Postgres, Transaction};

/// Creates a new post in the database with associated categories.
///
/// This function inserts a new post into the database and associates it with
/// multiple categories.
/// The insertion process is handled within a database transaction to ensure
/// atomicity.
/// After successfully inserting the post and its categories, the transaction
/// is committed, and the complete post with all its data is retrieved from
/// the database and returned as a `PostDTO`.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post` - A `Post` struct containing the data of the post to be created.
/// * `categories_ids` - A vector of category IDs (`Vec<i32>`) to associate
/// with the post.
///
/// # Returns
///
/// Returns a `Result` containing a `PostDTO` if the post creation is
/// successful, or a `sqlx::Error` if there is an error during the transaction
/// or query execution.
///
/// # Errors
///
/// This function will return an error if there is an issue executing any of
/// the SQL queries or committing the transaction.
pub async fn create_post_service(
    pool: &PgPool,
    post: Post,
    categories_ids: Vec<i32>,
) -> Result<PostDTO, sqlx::Error> {
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // Insert the post into database and retrieve post ID
    let post_id = insert_post(&mut transaction, &post).await?;

    let mut categories = Vec::new();

    // Insert associated categories
    for category_id in categories_ids {
        let post_category = PostsCategories {
            id: None,
            post_id,
            category_id: category_id,
            date_created: None,
        };

        insert_posts_categories(
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
    let post = select_post_by_id(pool, post_id).await?;

    Ok(post)
}
