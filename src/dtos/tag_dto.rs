use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::{
    handlers::generate_slug_handler::generate_slug,
    models::tags_model::TagModel, validators::slug_validator::validate_slug,
};

/// Batch deletion of tags
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteTagIdsDTO {
    pub ids: Vec<i32>,
}

/// Creating a tag
#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreateTagDTO {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

/// Converts `CreateTagDTO` to `TagModel`
impl TryFrom<CreateTagDTO> for TagModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreateTagDTO) -> Result<Self, Self::Error> {
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

        let tag = TagModel {
            id: None,
            name: dto.name,
            slug: Some(slug),
            description: dto.description,
            date_created: None,
        };

        tag.validate()?;
        Ok(tag)
    }
}

/// Full tag data
#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct TagDTO {
    pub id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_created: Option<NaiveDateTime>,
}

/// Converts `TagModel` to `TagDTO`
impl From<TagModel> for TagDTO {
    fn from(tag: TagModel) -> Self {
        TagDTO {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
            description: tag.description,
            date_created: tag.date_created,
        }
    }
}

/// Converts `TagDTO` to `TagModel`
impl TryFrom<TagDTO> for TagModel {
    type Error = ValidationErrors;

    fn try_from(dto: TagDTO) -> Result<Self, Self::Error> {
        let tag = TagModel {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            date_created: dto.date_created,
        };

        tag.validate()?;
        Ok(tag)
    }
}
