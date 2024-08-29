use serde::Serialize;

#[derive(Serialize)]
pub struct CategoryInfo {
    pub category_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct CategoryId {
    pub id: i32,
}
