use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostsCategoriesDTO {
    pub id: Option<i32>,
    pub post_id: i32,
    pub category_id: i32,
    pub date_created: Option<NaiveDateTime>,
}
