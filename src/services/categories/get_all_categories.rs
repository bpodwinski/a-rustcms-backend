use sqlx::PgPool;

use crate::{
    dtos::category_dto::CategoryDTO,
    repositories::categories::select_all_categories::select_all_categories,
};

pub async fn get_all_categories_service(
    pool: &PgPool,
) -> Result<Vec<CategoryDTO>, sqlx::Error> {
    let categories = select_all_categories(pool).await?;

    Ok(categories)
}
