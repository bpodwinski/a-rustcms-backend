use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::models::posts_model::{PostModel, PostsStatus};

/// Batch deletion of post
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeletePostIdsDTO {
    pub ids: Vec<i32>,
}

#[derive(FromRow)]
pub struct PostId {
    pub id: i32,
}

/// Creating a post
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreatePostDTO {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_id: i32,
    pub status: PostsStatus,
    pub date_published: Option<NaiveDateTime>,
    pub categories_ids: Vec<i32>,
}

/// Converts `CreatePostDTO` to `PostModel`
impl TryFrom<CreatePostDTO> for PostModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreatePostDTO) -> Result<Self, Self::Error> {
        let post = PostModel {
            id: None,
            title: dto.title,
            content: dto.content,
            slug: dto.slug,
            author_id: dto.author_id,
            status: dto.status,
            date_published: dto.date_published,
            date_created: None,
            categories: None,
        };

        post.validate()?;
        Ok(post)
    }
}

/// Full post data
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct PostDTO {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_id: i32,
    pub status: PostsStatus,

    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_published: Option<NaiveDateTime>,

    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_created: Option<NaiveDateTime>,
    pub categories: Option<serde_json::Value>,
}

/// Converts `PostModel` to `PostDTO`
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

/// Converts `PostDTO` to `PostModel`
impl TryFrom<PostDTO> for PostModel {
    type Error = ValidationErrors;

    fn try_from(dto: PostDTO) -> Result<Self, Self::Error> {
        let tag = PostModel {
            id: dto.id,
            title: dto.title,
            content: dto.content,
            slug: dto.slug,
            author_id: dto.author_id,
            status: dto.status,
            date_published: dto.date_published,
            date_created: dto.date_created,
            categories: dto.categories,
        };

        tag.validate()?;
        Ok(tag)
    }
}
