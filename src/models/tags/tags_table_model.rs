use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::validators::slug_validator::validate_slug;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagModel {
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
        max = 100,
        message = "Name  must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(custom(function = "validate_slug_tag"))]
    pub slug: String,

    pub description: Option<String>,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}

fn validate_slug_tag(slug: &str) -> Result<(), ValidationError> {
    let min_length = 0;
    let max_length = 200;

    validate_slug(&slug, min_length, max_length)
}
