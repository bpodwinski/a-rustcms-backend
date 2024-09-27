use anyhow::Result;
use sqlx::PgPool;

use crate::{
    models::tags::tags_table_model::TagModel,
    repositories::tags_repository::select_tags,
};

pub async fn get_all_tags_service(
    pool: &PgPool,
) -> Result<Vec<TagModel>, sqlx::Error> {
    let tag_entities = select_tags(pool).await?;

    let result_dto: Vec<TagModel> = tag_entities
        .into_iter()
        .map(|tag_entity| TagModel {
            id: tag_entity.id,
            name: tag_entity.name,
            slug: tag_entity.slug,
            description: tag_entity.description,
            date_created: tag_entity.date_created,
        })
        .collect();

    Ok(result_dto)
}
