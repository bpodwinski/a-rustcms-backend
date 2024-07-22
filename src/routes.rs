use ntex::web;

use crate::handlers::post::{create_post, delete_post_by_id, get_post_by_id, get_posts, update_post_by_id};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/posts", web::get().to(get_posts))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}", web::get().to(get_post_by_id))
            .route("/posts/{id}", web::delete().to(delete_post_by_id))
            .route("/posts/{id}", web::put().to(update_post_by_id)),
    );
}
