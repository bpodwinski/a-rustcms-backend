use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "posts_status")]
pub enum PostsStatus {
    Draft,
    Pending,
    Private,
    Scheduled,
    Published,
}
