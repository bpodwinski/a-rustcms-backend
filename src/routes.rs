use ntex::web;

use crate::controllers::{
    posts::create_post::create_post_controller,
    posts::get_all_posts::get_all_posts_controller,
    posts::update_post::update_post_controller,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            //.service(get_posts)
            .service(create_post_controller)
            .service(get_all_posts_controller)
            .service(update_post_controller), //.service(delete_post_by_id)
    );
}
