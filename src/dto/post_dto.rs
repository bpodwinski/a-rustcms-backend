use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::posts::posts_table_model::Post;
use crate::models::posts::posts_type_model::PostsStatus;

#[derive(sqlx::FromRow)]
pub struct PostId {
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostDTO {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub status: PostsStatus,
    pub date_published: Option<NaiveDateTime>,
    pub date_created: Option<NaiveDateTime>,
    pub categories: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CategoryDTO {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<Post> for PostDTO {
    fn from(post: Post) -> Self {
        PostDTO {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            status: post.status,
            date_published: post.date_published,
            date_created: post.date_created,
            categories: post.categories,
        }
    }
}
