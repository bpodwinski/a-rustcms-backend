use ntex::web::{self, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::models::categories::categories_table_model::Category;
use crate::services::categories::create_category::create_category_service;

/// Create a new post in the database.
///
/// This function inserts a new row into the `posts` table in the database
/// with the provided title, content, and author ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `new_post` - A JSON object containing the new post data.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation.
/// If successful, it returns a `Created` status.
#[web::post("/categories")]
pub async fn create_category_controller(
    pool: web::types::State<PgPool>,
    new_category: web::types::Json<Category>,
) -> HttpResponse {
    if let Err(errors) = new_category.validate() {
        return HttpResponse::BadRequest().json(&errors);
    }

    match create_category_service(&pool, new_category.into_inner()).await {
        Ok(category) => HttpResponse::Created().json(&category),
        Err(err) => {
            eprintln!("Failed to create post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
