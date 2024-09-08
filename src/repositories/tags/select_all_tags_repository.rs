use anyhow::Result;
use sqlx::PgPool;

use crate::models::tags::tags_table_model::TagModel;

/// Fetches all tags from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool used to execute the query.
///
/// # Returns
///
/// A result containing a vector of `TagModel` on success,
/// or a `sqlx::Error` on failure.
///
/// # Errors
///
/// This function returns an error if the database query fails. The error type is `sqlx::Error`,
/// which includes various kinds of database-related errors.
pub async fn select(pool: &PgPool) -> Result<Vec<TagModel>, sqlx::Error> {
    let rows = sqlx::query_file!("src/repositories/tags/select_all_tags.sql")
        .fetch_all(pool)
        .await?;

    let tags = rows
        .into_iter()
        .map(|row| TagModel {
            id: Some(row.id),
            name: row.name,
            slug: row.slug,
            description: Some(row.description.unwrap_or_default()),
            date_created: Some(row.date_created),
        })
        .collect();

    Ok(tags)
}
