use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::TagDTO, repositories::tags::select_all_tags_repository,
};

pub async fn get_all_tags_service(
    pool: &PgPool,
) -> Result<Vec<TagDTO>, sqlx::Error> {
    let tag_entities = select_all_tags_repository::select(pool).await?;

    let result_dto: Vec<TagDTO> = tag_entities
        .into_iter()
        .map(|tag_entity| TagDTO {
            id: tag_entity.id,
            name: tag_entity.name,
            slug: tag_entity.slug,
            description: tag_entity.description,
            date_created: tag_entity.date_created,
        })
        .collect();

    Ok(result_dto)
}
