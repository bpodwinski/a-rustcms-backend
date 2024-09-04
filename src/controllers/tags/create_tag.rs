use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

use crate::{
    dto::tag_dto::TagDTO, services::tags::create_tag::create_tag_service,
};

#[web::post("/tags")]
pub async fn create_tag_controller(
    pool: web::types::State<PgPool>,
    new_tag: web::types::Json<TagDTO>,
) -> HttpResponse {
    match create_tag_service(&pool, &new_tag.into_inner()).await {
        Ok(tag) => HttpResponse::Created().json(&tag),
        Err(err) => {
            eprintln!("Failed to create tag: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
