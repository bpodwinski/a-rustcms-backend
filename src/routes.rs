use ntex::web;

use crate::{
    controllers::{
        categories::{
            create_category_controller::create_category_controller,
            delete_category_controller::delete_category_controller,
            get_all_categories_controller::get_all_categories_controller,
            get_category_by_id_controller::get_category_by_id_controller,
            update_category_controller::update_category_controller,
        },
        posts::{
            create_post_controller::create_post_controller,
            delete_post_controller::delete_post_controller,
            get_all_posts_controller::get_all_posts_controller,
            get_post_by_id_controller::get_post_by_id_controller,
            update_post_controller::update_post_controller,
        },
        tags::{
            create_tag_controller::create_tag_controller,
            delete_tag_controller::delete_tag_controller,
            get_all_tags_controller::get_all_tags_controller,
            get_tag_by_id_controller::get_tag_by_id_controller,
        },
    },
    middlewares::jwt_middleware::JwtMiddleware,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            //.wrap(JwtMiddleware)
            .service(create_tag_controller)
            .service(get_all_tags_controller)
            .service(get_tag_by_id_controller)
            .service(delete_tag_controller)
            .service(create_category_controller)
            .service(get_all_categories_controller)
            .service(get_category_by_id_controller)
            .service(delete_category_controller)
            .service(update_category_controller)
            .service(create_post_controller)
            .service(get_all_posts_controller)
            .service(get_post_by_id_controller)
            .service(update_post_controller)
            .service(delete_post_controller),
    );
}
