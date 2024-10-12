use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use ntex::http;
use ntex::web::{self, guard::Guard};

use crate::config::config::get_secret_key;
use crate::controllers::{
    auth::login_controller::login_controller,
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
        update_tag_controller::update_tag_controller,
    },
    users::{
        create_user_controller::create_user_controller,
        delete_user_controller::delete_user_controller,
        get_user_by_id_controller::get_user_by_id_controller,
        update_user_controller::update_user_controller,
    },
};
use crate::dtos::auth_dtos::ClaimsDTO;

struct JwtGuard;

impl Guard for JwtGuard {
    fn check(&self, req: &http::RequestHead) -> bool {
        if let Some(auth_header) =
            req.headers().get(http::header::AUTHORIZATION)
        {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];

                    let decoding_key =
                        DecodingKey::from_secret(get_secret_key().as_ref());
                    let validation = Validation::new(Algorithm::HS256);

                    if decode::<ClaimsDTO>(token, &decoding_key, &validation)
                        .is_ok()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1").service(login_controller).service(
            web::scope("/")
                .guard(JwtGuard)
                .service(create_tag_controller)
                .service(update_tag_controller)
                .service(get_all_tags_controller)
                .service(get_tag_by_id_controller)
                .service(delete_tag_controller)
                .service(create_category_controller)
                .service(get_all_categories_controller)
                .service(get_category_by_id_controller)
                .service(update_category_controller)
                .service(delete_category_controller)
                .service(create_post_controller)
                .service(update_post_controller)
                .service(get_all_posts_controller)
                .service(get_post_by_id_controller)
                .service(delete_post_controller)
                .service(create_user_controller)
                .service(update_user_controller)
                .service(get_user_by_id_controller)
                .service(delete_user_controller),
        ),
    );
}
