use ntex::web::{self, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;

use crate::services::posts::delete_post::delete_post_service;

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

#[web::delete("/posts/{id}")]
pub async fn delete_post_controller(
    pool: web::types::State<PgPool>,
    post_id: web::types::Path<i32>,
) -> HttpResponse {
    match delete_post_service(pool.get_ref(), post_id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(&ResponseMessage {
                message: "Post deleted successfully".to_string(),
            })
        }
        Ok(_) => HttpResponse::NotFound().body("Post not found"),
        Err(err) => {
            eprintln!("Failed to delete post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
