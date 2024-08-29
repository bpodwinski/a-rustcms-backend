use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    dto::ErrorMessage, services::posts::delete_post::delete_post_service,
};

#[web::delete("/posts/{id}")]
pub async fn delete_post_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> HttpResponse {
    match delete_post_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::NoContent().finish()
        }
        Ok(_) => HttpResponse::NotFound().json(&ErrorMessage {
            error: "Post not found".to_string(),
        }),
        Err(err) => {
            eprintln!("Failed to delete post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
