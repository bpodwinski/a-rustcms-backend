use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::validators::slug_validator::validate_slug;

use super::posts_type_model::PostsStatus;

/// Represents a blog post with associated metadata and categories.
#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostModel {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Title must be between 1 and 500 characters"
    ))]
    pub title: String,

    #[validate(length(
        max = 5000,
        message = "Content cannot exceed 5000 characters"
    ))]
    pub content: String,

    #[validate(custom(function = "validate_slug_post"))]
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
        PostsStatus::Draft
        | PostsStatus::Pending
        | PostsStatus::Published
        | PostsStatus::Private
        | PostsStatus::Scheduled => Ok(()),
        _ => Err(ValidationError::new("Invalid post status")),
    }
}

fn validate_slug_post(slug: &str) -> Result<(), ValidationError> {
    let min_length = 1;
    let max_length = 500;

    validate_slug(&slug, min_length, max_length)
}
