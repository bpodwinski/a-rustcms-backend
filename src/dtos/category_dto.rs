use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::{
    handlers::generate_slug_handler::generate_slug,
    models::categories_model::CategoryModel,
    validators::slug_validator::validate_slug,
};

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
        let mut errors = ValidationErrors::new();
        let slug = dto.slug.unwrap_or_else(|| generate_slug(&dto.name));
        let min_length = 1;
        let max_length = 200;
        if let Err(validation_error) =
            validate_slug(&slug, min_length, max_length)
        {
            errors.add("slug", validation_error.into());
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        let category = CategoryModel {
            id: None,
            parent_id: dto.parent_id,
            name: dto.name.trim().to_string(),
            slug: Some(slug),
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
