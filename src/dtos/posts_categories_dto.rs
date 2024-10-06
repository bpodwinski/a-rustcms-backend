use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct PostsCategoriesDTO {
    pub id: Option<i32>,
    pub post_id: i32,
    pub category_id: i32,
    pub date_created: Option<NaiveDateTime>,
}
