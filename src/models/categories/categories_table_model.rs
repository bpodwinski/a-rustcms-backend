use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[validate(range(min = 1, message = "Parent ID must be greater than 0."))]
    pub parent_id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters."
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Description must be between 1 and 500 characters."
    ))]
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
