use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostsCategories {
    #[serde(skip_serializing_if = "Option::is_none")]
    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub id: Option<i32>,

    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub post_id: i32,

    // https://www.postgresql.org/docs/8.1/datatype.html#DATATYPE-NUMERIC
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Author ID must be between 1 and 2,147,483,647"
    ))]
    pub category_id: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDateTime>,
}
