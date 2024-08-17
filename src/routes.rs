use ntex::web;

use crate::controllers::{
    categories::create_category::create_category_controller,
    posts::{
        create_post::create_post_controller,
        delete_post::delete_post_controller,
        get_all_posts::get_all_posts_controller,
        update_post::update_post_controller,
    },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(create_post_controller)
            .service(get_all_posts_controller)
            .service(update_post_controller)
            .service(delete_post_controller)
            .service(create_category_controller),
    );
}
