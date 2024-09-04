use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents a tag
#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters."
    ))]
    pub name: String,

    pub slug: Option<String>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Description must be between 1 and 500 characters"
    ))]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
