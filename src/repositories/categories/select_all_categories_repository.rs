use anyhow::Result;
use sqlx::PgPool;

use crate::{dtos::category_dto::CategoryDTO, repositories::QueryBuilder};

pub async fn select(pool: &PgPool) -> Result<Vec<CategoryDTO>, sqlx::Error> {
    let result = QueryBuilder::<CategoryDTO>::new(&pool)
        .table("categories")
        .fields(&[
            "id",
            "parent_id",
            "name",
            "slug",
            "description",
            "date_created",
        ])
        .select()
        .await;

    result
}
