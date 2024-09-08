use anyhow::Result;
use sqlx::PgPool;
use validator::Validate;

use crate::handlers::error_handler::ServiceError;
use crate::handlers::generate_slug_handler::generate_slug;
use crate::{
    dtos::tag_dto::TagDTO, models::tags::tags_table_model::TagModel,
    repositories::tags::insert_tag_repository,
};

pub async fn create_tag_service(
    pool: &PgPool,
    mut tag_dto: TagDTO,
) -> Result<TagDTO, ServiceError> {
    if tag_dto.slug.is_empty() {
        tag_dto.slug = generate_slug(&tag_dto.name);
    }

    let tag_model = TagModel {
        id: None,
        name: tag_dto.name.clone(),
        slug: tag_dto.slug.clone(),
        description: tag_dto.description.clone(),
        date_created: None,
    };

    tag_model.validate()?;

    let tag_entity = insert_tag_repository::insert(pool, tag_model).await?;

    let result_dto = TagDTO {
        id: tag_entity.id,
        name: tag_entity.name,
        slug: tag_entity.slug,
        description: tag_entity.description,
        date_created: tag_entity.date_created,
    };

    Ok(result_dto)
}
