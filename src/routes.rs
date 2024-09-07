use ntex::web;

use crate::controllers::{
    categories::create_category_controller::create_category_controller,
    tags::create_tag_controller::create_tag_controller,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(create_tag_controller) //.service(create_post_controller)
            //.service(get_all_posts_controller)
            //.service(get_post_by_id_controller) //.service(update_post_controller)
            //.service(delete_post_controller)
            .service(create_category_controller), //.service(get_all_categories_controller),
    );
}
