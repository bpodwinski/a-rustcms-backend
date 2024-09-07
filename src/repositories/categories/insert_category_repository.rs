use sqlx::PgPool;

use crate::models::categories::categories_table_model::CategoryModel;

pub async fn insert(
    pool: &PgPool,
    category_model: CategoryModel,
) -> Result<CategoryModel, sqlx::Error> {
    let category = sqlx::query_file_as!(
        CategoryModel,
        "src/repositories/categories/insert_category.sql",
        category_model.parent_id,
        category_model.name,
        category_model.slug,
        category_model.description
    )
    .fetch_one(pool)
    .await?;

    Ok(category)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db, models::categories::categories_table_model::CategoryModel,
    };
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_insert_category() {
        //Arrange
        dotenv().ok();
        let pool = db::init_pool().await.expect("Failed to create pool");

        let new_category = CategoryModel {
            id: None,
            parent_id: None,
            name: "Test Category".to_string(),
            slug: "test-category".to_string(),
            description: Some("A test category description".to_string()),
            date_created: None,
        };

        // Act
        let result = insert(&pool, new_category).await;

        // Assert
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Test Category");
        assert_eq!(category.slug, "test-category");
        assert_eq!(
            category.description.unwrap(),
            "A test category description"
        );
    }
}
