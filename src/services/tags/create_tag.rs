use anyhow::{Context, Result};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    dto::tag_dto::TagDTO, models::tags::tags_table_model::TagModel,
    repositories::tags::insert_tag::insert_tag_repository,
};

pub async fn create_tag_service(
    pool: &PgPool,
    tag_dto: TagDTO,
) -> Result<TagDTO> {
    let tag_model = TagModel {
        id: None,
        name: tag_dto.name.clone(),
        slug: tag_dto.slug.clone(),
        description: tag_dto.description.clone(),
        date_created: None,
    };

    tag_model
        .validate()
        .context("Validation failed for TagModel")?;

    let tag_entity = insert_tag_repository(pool, tag_model).await?;

    let result_dto = TagDTO {
        id: tag_entity.id,
        name: tag_entity.name,
        slug: tag_entity.slug,
        description: tag_entity.description,
        date_created: None,
    };

    Ok(result_dto)
}
