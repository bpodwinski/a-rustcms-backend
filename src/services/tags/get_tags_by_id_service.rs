use anyhow::Result;
use sqlx::*;

use crate::{
    models::tags::tags_table_model::TagModel,
    repositories::tags_repository::select_tag_by_id,
};

pub async fn get_tag_by_id_service(
    pool: &PgPool,
    tag_id: i32,
) -> Result<Option<TagModel>, Error> {
    let tag_entity = select_tag_by_id(pool, tag_id).await?;

    let result_dto = TagModel {
        id: tag_entity.id,
        name: tag_entity.name,
        slug: tag_entity.slug,
        description: tag_entity.description,
        date_created: tag_entity.date_created,
    };

    Ok(Some(result_dto))
}
