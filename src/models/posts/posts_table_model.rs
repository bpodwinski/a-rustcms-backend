use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::posts_type_model::Status;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    #[validate(range(min = 1, message = "Author ID must be greater than 0"))]
    pub author_id: i32,

    pub status: Status,

    pub date_published: Option<NaiveDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
