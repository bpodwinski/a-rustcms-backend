use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_id: i32,
}
