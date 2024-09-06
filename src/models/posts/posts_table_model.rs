use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::posts_type_model::PostsStatus;

/// Represents a blog post with associated metadata and categories.
#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub id: Option<i32>,

    pub title: String,

    #[validate(length(
        max = 5000,
        message = "Content cannot exceed 5000 characters"
    ))]
    pub content: String,

    #[validate(length(
        min = 1,
        max = 200,
        message = "Slug must be between 1 and 200"
    ))]
    pub slug: String,

    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub author_id: i32,

    #[validate(custom(function = "validate_post_status"))]
    pub status: PostsStatus,

    pub date_published: Option<NaiveDateTime>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub categories: Option<serde_json::Value>,
}

fn validate_post_status(status: &PostsStatus) -> Result<(), ValidationError> {
    match status {
        PostsStatus::Draft | PostsStatus::Published => Ok(()),
        _ => Err(ValidationError::new("Invalid post status")),
    }
}
