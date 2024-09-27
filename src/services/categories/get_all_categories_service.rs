use anyhow::Result;
use sqlx::*;

use crate::{
    dtos::category_dto::CategoryDTO,
    repositories::categories_repository::select_categories,
};

pub async fn get_all_categories_service(
    pool: &PgPool,
) -> Result<Vec<CategoryDTO>, Error> {
    let categories_model = select_categories(pool).await?;

    let categories_dto = categories_model
        .into_iter()
        .map(|row| CategoryDTO {
            id: row.id,
            parent_id: row.parent_id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            date_created: row.date_created,
        })
        .collect();

    Ok(categories_dto)
}
