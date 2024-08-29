use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::posts::posts_type_model::PostsStatus;

use super::category_dto::CategoryInfo;

#[derive(Serialize)]
pub struct PostWithCategories {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub status: PostsStatus,
    pub date_published: Option<NaiveDateTime>,
    pub date_created: Option<NaiveDateTime>,
    pub categories: Vec<CategoryInfo>,
}

#[derive(sqlx::FromRow)]
pub struct PostId {
    pub id: i32,
}
