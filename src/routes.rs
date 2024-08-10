use ntex::web;

use crate::controllers::posts::create_post::create_post_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            //.service(get_posts)
            .service(create_post_controller), //.service(get_post_by_id)
                                              //.service(delete_post_by_id)
                                              //.service(update_post_by_id),
    );
}
