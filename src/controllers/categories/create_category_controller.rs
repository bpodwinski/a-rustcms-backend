use ntex::web::{self, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::models::categories::categories_table_model::Category;
use crate::services::categories::create_category_service::create_category_service;

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
