use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::categories::categories_table_model::Category;

#[derive(sqlx::FromRow)]
pub struct CategoryId {
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct CategoryDTO {
    pub id: Option<i32>,
    pub parent_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub date_created: Option<NaiveDateTime>,
}

impl From<Category> for CategoryDTO {
    fn from(category: Category) -> Self {
        CategoryDTO {
            id: category.id,
            parent_id: category.parent_id,
            name: category.name,
            description: category.description,
            date_created: category.date_created,
        }
    }
}
