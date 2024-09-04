use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::tags::tags_table_model::Tag;

#[derive(sqlx::FromRow)]
pub struct TagId {
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct TagDTO {
    pub id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub date_created: Option<NaiveDateTime>,
}

impl From<Tag> for TagDTO {
    fn from(tag: Tag) -> Self {
        TagDTO {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
            description: tag.description,
            date_created: tag.date_created,
        }
    }
}
