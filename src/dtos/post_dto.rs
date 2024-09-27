use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::posts::posts_table_model::PostModel;
use crate::models::posts::posts_type_model::PostsStatus;

#[derive(sqlx::FromRow)]
pub struct PostId {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct CreatePostDTO {
    pub post: PostModel,
    pub categories_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePostsIdsDTO {
    pub ids: Vec<i32>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostDTO {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_id: i32,
    pub status: PostsStatus,
    pub date_published: Option<NaiveDateTime>,
    pub date_created: Option<NaiveDateTime>,
    pub categories: Option<serde_json::Value>,
}

impl From<PostModel> for PostDTO {
    fn from(post: PostModel) -> Self {
        PostDTO {
            id: post.id,
            title: post.title,
            content: post.content,
            slug: post.slug,
            author_id: post.author_id,
            status: post.status,
            date_published: post.date_published,
            date_created: post.date_created,
            categories: post.categories,
        }
    }
}

pub enum SortColumn {
    Id,
    Title,
    AuthorId,
    DatePublished,
}

pub enum SortOrder {
    Asc,
    Desc,
}
