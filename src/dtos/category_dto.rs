use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::categories::categories_table_model::CategoryModel;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct CategoryDTO {
    pub id: Option<i32>,
    pub parent_id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub date_created: Option<NaiveDateTime>,
}

impl From<CategoryModel> for CategoryDTO {
    fn from(category: CategoryModel) -> Self {
        CategoryDTO {
            id: category.id,
            parent_id: category.parent_id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            date_created: category.date_created,
        }
    }
}
