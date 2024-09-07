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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, models::tags::tags_table_model::TagModel};
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_insert_tag() {
        //Arrange
        dotenv().ok();
        let pool = db::init_pool().await.expect("Failed to create pool");

        let new_tag = TagModel {
            id: None,
            name: "Test Tag".to_string(),
            slug: "test-tag".to_string(),
            description: Some("A test tag description".to_string()),
            date_created: None,
        };

        // Act
        let result = insert(&pool, new_tag).await;

        // Assert
        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.name, "Test Tag");
        assert_eq!(tag.slug, "test-tag");
        assert_eq!(tag.description.unwrap(), "A test tag description");
    }
}
