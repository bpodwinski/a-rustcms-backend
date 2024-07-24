use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "posts_status")]
pub enum StatusEnum {
    Draft,
    Pending,
    Private,
    Scheduled,
    Published,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct PostStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub status: StatusEnum,
    pub date_published: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewPostStruct {
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub status: StatusEnum,
    pub date_published: Option<NaiveDateTime>,
}
