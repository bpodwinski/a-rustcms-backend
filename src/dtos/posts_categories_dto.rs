use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::models::posts_categories_model::PostsCategoriesModel;

/// Batch deletion of posts categories
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeletePostsCategoriesIdsDTO {
    pub ids: Vec<i32>,
}

/// Creating a post categorie
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreatePostsCategoriesDTO {
    pub post_id: i32,
    pub category_id: i32,
}

/// Converts `CreateCategoryDTO` to `CategoryModel`
impl TryFrom<CreatePostsCategoriesDTO> for PostsCategoriesModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreatePostsCategoriesDTO) -> Result<Self, Self::Error> {
        let posts_categories = PostsCategoriesModel {
            id: None,
            post_id: dto.post_id,
            category_id: dto.category_id,
            date_created: None,
        };

        posts_categories.validate()?;
        Ok(posts_categories)
    }
}

#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct PostsCategoriesDTO {
    pub id: Option<i32>,
    pub post_id: i32,
    pub category_id: i32,
    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_created: Option<NaiveDateTime>,
}

/// Converts `PostsCategoriesModel` to `PostsCategoriesDTO`
impl From<PostsCategoriesModel> for PostsCategoriesDTO {
    fn from(posts_categories: PostsCategoriesModel) -> Self {
        PostsCategoriesDTO {
            id: posts_categories.id,
            post_id: posts_categories.post_id,
            category_id: posts_categories.category_id,
            date_created: posts_categories.date_created,
        }
    }
}

/// Converts `PostsCategoriesDTO` to `PostsCategoriesModel`
impl TryFrom<PostsCategoriesDTO> for PostsCategoriesModel {
    type Error = ValidationErrors;

    fn try_from(dto: PostsCategoriesDTO) -> Result<Self, Self::Error> {
        let posts_categories = PostsCategoriesModel {
            id: dto.id,
            post_id: dto.post_id,
            category_id: dto.category_id,
            date_created: dto.date_created,
        };

        posts_categories.validate()?;
        Ok(posts_categories)
    }
}
