use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostsCategories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[validate(range(min = 1, message = "Author ID must be greater than 0"))]
    pub post_id: i32,

    #[validate(range(min = 1, message = "Author ID must be greater than 0"))]
    pub category_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
