use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, FromRow, Clone)]
pub struct UserModel {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "ID must be between 1 and 2,147,483,647"
    ))]
    pub id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 60,
        message = "Login must be between 1 and 60 characters"
    ))]
    pub login: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub password: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Email must be between 1 and 100 characters"
    ))]
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "First name must be between 1 and 100 characters"
    ))]
    pub firstname: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Last name must be between 1 and 100 characters"
    ))]
    pub lastname: String,

    #[validate(length(
        max = 255,
        message = "URL must be a maximum of 255 characters"
    ))]
    #[validate(url(message = "URL must be a valid URL"))]
    pub url: String,

    pub active: bool,

    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
