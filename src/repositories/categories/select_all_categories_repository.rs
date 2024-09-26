use anyhow::Result;
use sqlx::*;

use crate::{dtos::category_dto::CategoryDTO, repositories::QueryBuilder};

pub async fn select(pool: &PgPool) -> Result<Vec<CategoryDTO>, Error> {
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
