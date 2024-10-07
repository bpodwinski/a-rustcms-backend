use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, Debug, ToSchema, IntoParams)]
pub struct PaginationParamsDTO {
    #[validate(range(
        min = 1,
        max = 2_147_483_647,
        message = "Limit must be between 1 and 2,147,483,647"
    ))]
    #[schema(example = 20)]
    pub limit: Option<i64>,

    #[validate(range(
        min = 0,
        max = 2_147_483_647,
        message = "Offset must be between 0 and 2,147,483,647"
    ))]
    #[schema(example = 1)]
    pub page: Option<i64>,

    #[schema(example = "id")]
    pub sort_column: Option<String>,

    #[schema(example = "asc")]
    pub sort_order: Option<String>,
}

#[derive(Validate, Serialize, Deserialize, FromRow, Debug, ToSchema)]
pub struct PaginationDTO<T> {
    pub current_page: i64,
    pub total_pages: i64,
    pub total_items: i64,
    pub data: Vec<T>,
}
