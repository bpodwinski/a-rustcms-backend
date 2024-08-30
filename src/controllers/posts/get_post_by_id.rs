use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    dto::ErrorMessage, services::posts::get_post_by_id::get_post_by_id_service,
};

#[web::get("/posts/{id}")]
pub async fn get_post_by_id_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> HttpResponse {
    match get_post_by_id_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(&post),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(&ErrorMessage {
                error: "Post not found".to_string(),
            })
        }
        Err(err) => {
            eprintln!("Failed to retrieve post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
