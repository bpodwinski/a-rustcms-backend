use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::tag_dto::{CreateTagDTO, DeleteTagIdsDTO};
use crate::models::tags_model::TagModel;
use crate::repositories::tags_repository::{
    delete_tag_by_id, insert_tag, select_tag_by_id, select_tags, update_tag,
};

/// Service pour insérer un tag dans la base de données.
pub async fn create_tag_service(
    pool: &PgPool,
    create_tag_dto: CreateTagDTO,
) -> Result<TagModel> {
    let tag_model: TagModel = create_tag_dto.try_into()?;

    let result = insert_tag(pool, tag_model).await?;
    Ok(result)
}

/// Service pour mettre à jour un tag par son ID dans la base de données.
pub async fn update_tag_service(
    pool: &PgPool,
    id: i32,
    tag_model: TagModel,
) -> Result<TagModel> {
    let updated_tag = update_tag(pool, id, tag_model).await?;
    Ok(updated_tag)
}

/// Service pour récupérer tous les tags dans la base de données.
pub async fn get_all_tags_service(pool: &PgPool) -> Result<Vec<TagModel>> {
    let tags = select_tags(pool).await?;
    Ok(tags)
}

/// Service pour récupérer un tag par son ID dans la base de données.
pub async fn get_tag_by_id_service(pool: &PgPool, id: i32) -> Result<TagModel> {
    let tag = select_tag_by_id(pool, id).await?;
    Ok(tag)
}

/// Service pour supprimer des tags par ID dans la base de données.
pub async fn delete_tag_by_id_service(
    pool: &PgPool,
    delete_tag_ids_dto: DeleteTagIdsDTO,
) -> Result<Vec<i32>> {
    let deleted_ids = delete_tag_by_id(pool, delete_tag_ids_dto.ids).await?;
    Ok(deleted_ids)
}
