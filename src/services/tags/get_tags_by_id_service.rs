use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::TagDTO, repositories::tags::select_tag_by_id_repository,
};

pub async fn get_tag_by_id_service(
    pool: &PgPool,
    tag_id: i32,
) -> Result<Option<TagDTO>, sqlx::Error> {
    let tag_entity = select_tag_by_id_repository::select(pool, tag_id).await?;

    let result_dto = TagDTO {
        id: tag_entity.id,
        name: tag_entity.name,
        slug: tag_entity.slug,
        description: tag_entity.description,
        date_created: tag_entity.date_created,
    };

    Ok(Some(result_dto))
}
