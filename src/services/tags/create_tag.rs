use sqlx::PgPool;

use crate::{dto::tag_dto::TagDTO, repositories::tags::insert_tag::insert_tag};

pub async fn create_tag_service(
    pool: &PgPool,
    tag: &TagDTO,
) -> Result<i32, sqlx::Error> {
    let tag_id = insert_tag(pool, tag).await?;
    Ok(tag_id)
}
