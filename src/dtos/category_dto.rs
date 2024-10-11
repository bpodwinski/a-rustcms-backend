use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::models::categories_model::CategoryModel;

/// Batch deletion of categories
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteCategoryIdsDTO {
    pub ids: Vec<i32>,
}

/// Creating a category
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryDTO {
    pub parent_id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Converts `CreateCategoryDTO` to `CategoryModel`
impl TryFrom<CreateCategoryDTO> for CategoryModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreateCategoryDTO) -> Result<Self, Self::Error> {
        let category = CategoryModel {
            id: None,
            parent_id: dto.parent_id,
            name: dto.name.trim().to_string(),
            slug: dto.slug,
            description: dto.description.map(|desc| desc.trim().to_string()),
            date_created: None,
        };

        category.validate()?;
        Ok(category)
    }
}

/// Full category data
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoryDTO {
    pub id: Option<i32>,
    pub parent_id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_created: Option<NaiveDateTime>,
}

/// Converts `CategoryModel` to `CategoryDTO`
impl From<CategoryModel> for CategoryDTO {
    fn from(category: CategoryModel) -> Self {
        CategoryDTO {
            id: category.id,
            parent_id: category.parent_id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            date_created: category.date_created,
        }
    }
}

/// Converts `CategoryDTO` to `CategoryModel`
impl TryFrom<CategoryDTO> for CategoryModel {
    type Error = ValidationErrors;

    fn try_from(dto: CategoryDTO) -> Result<Self, Self::Error> {
        let category = CategoryModel {
            id: dto.id,
            parent_id: dto.parent_id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            date_created: dto.date_created,
        };

        category.validate()?;
        Ok(category)
    }
}
