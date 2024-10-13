use anyhow::Result;
use sqlx::PgPool;

use crate::{
    dtos::posts_categories_dto::{
        CreatePostsCategoriesDTO, PostsCategoriesDTO,
    },
    models::posts_categories_model::PostsCategoriesModel,
    repositories::posts_categories_repository::insert_post_category,
};

pub async fn create_post_category_service(
    pool: &PgPool,
    create_dto: CreatePostsCategoriesDTO,
) -> Result<PostsCategoriesDTO> {
    let model: PostsCategoriesModel = create_dto.try_into()?;

    let create_model = insert_post_category(pool, model).await?;
    let result = PostsCategoriesDTO::from(create_model);
    Ok(result)
}
