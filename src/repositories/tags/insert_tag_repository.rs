use sqlx::PgPool;

use crate::models::tags::tags_table_model::TagModel;

/// Inserts a tag into the database
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `tag_model` - The tag model containing the tag data to be inserted
///
/// # Returns
///
/// A Result containing the inserted TagModel or a sqlx::Error if something goes wrong
pub async fn insert(
    pool: &PgPool,
    tag_model: TagModel,
) -> Result<TagModel, sqlx::Error> {
    let tag = sqlx::query_file_as!(
        TagModel,
        "src/repositories/tags/insert_tag.sql",
        tag_model.name,
        tag_model.slug,
        tag_model.description
    )
    .fetch_one(pool)
    .await?;

    Ok(tag)
}
