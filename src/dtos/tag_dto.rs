use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::models::tags::tags_table_model::TagModel;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct TagDTO {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub date_created: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreateTagDTO {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

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

impl TryFrom<CreateTagDTO> for TagModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreateTagDTO) -> Result<Self, Self::Error> {
        let tag = TagModel {
            id: None,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            date_created: None,
        };

        tag.validate()?;
        Ok(tag)
    }
}
