use sqlx::PgPool;

use crate::{
    dto::post_dto::PostDTO, models::posts::posts_type_model::PostsStatus,
};

/// Retrieves all posts from the database and maps them to a list of PostDTOs.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `PostDTO` if the query is
/// successful, or a `sqlx::Error` if there is an error during the
/// query execution.
///
/// # Errors
///
/// This function will return an error if there is an issue executing
/// the SQL query.
///
pub async fn select_all_posts(
    pool: &PgPool,
) -> Result<Vec<PostDTO>, sqlx::Error> {
    // Execute the SQL query to fetch all posts
    let rows =
        sqlx::query_file!("src/repositories/posts/select_all_posts.sql",)
            .fetch_all(pool)
            .await?;

    // Map each row to a PostDTO and collect them into a Vec<PostDTO>
    let posts = rows
        .into_iter()
        .map(|row| PostDTO {
            id: Some(row.id),
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            status: PostsStatus::from(row.status),
            date_published: row.date_published,
            date_created: Some(row.date_created),
            categories: row.categories,
        })
        .collect();

    Ok(posts)
}
