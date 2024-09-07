use sqlx::PgPool;
use validator::Validate;

use crate::{
    dtos::category_dto::CategoryDTO, handlers::error_handler::ServiceError,
    models::categories::categories_table_model::CategoryModel,
    repositories::categories::insert_category_repository,
};

pub async fn create_category_service(
    pool: &PgPool,
    category_dto: CategoryModel,
) -> Result<CategoryDTO, ServiceError> {
    let category_model = CategoryModel {
        id: None,
        parent_id: category_dto.parent_id.clone(),
        name: category_dto.name.clone(),
        slug: category_dto.slug.clone(),
        description: category_dto.description.clone(),
        date_created: None,
    };

    category_model.validate()?;

    let category_entity =
        insert_category_repository::insert(pool, category_model).await?;

    let result_dto = CategoryDTO {
        id: category_entity.id,
        parent_id: category_entity.parent_id,
        name: category_entity.name,
        slug: category_entity.slug,
        description: category_entity.description,
        date_created: category_entity.date_created,
    };

    Ok(result_dto)
}
