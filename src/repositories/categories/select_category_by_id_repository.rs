use sqlx::PgPool;

use crate::{
    dtos::category_dto::CategoryDTO,
    models::posts::posts_type_model::PostsStatus,
};

/// Retrieves single post by its ID from the database and maps it to a PostDTO.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `post_id` - The ID of the post to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing a `PostDTO` if the post is found, or a
/// `sqlx::Error` if there is an error during the query.
///
/// # Errors
///
/// This function will return an error if the post with the specified
/// `post_id`does not exist, or if there is an issue executing the SQL query.
///
pub async fn select(
    pool: &PgPool,
    category_id: i32,
) -> Result<CategoryDTO, sqlx::Error> {
    // Execute SQL query to fetch the post with the specified ID
    let row = sqlx::query_file!(
        "src/repositories/categories/select_category_by_id.sql",
        category_id
    )
    .fetch_one(pool)
    .await?;

    // Map the result to a PostDTO
    let category = CategoryDTO {
        id: Some(row.id),
        parent_id: row.parent_id,
        name: row.name,
        description: Some(row.description.unwrap_or_else(|| String::new())),
        date_created: Some(row.date_created),
    };

    Ok(category)
}
