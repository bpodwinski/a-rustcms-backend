use ntex::util::Bytes;
use ntex::web;
use std::sync::Arc;
use utoipa::OpenApi;

use crate::{
    dtos::{
        category_dto::{CategoryDTO, CreateCategoryDTO, DeleteCategoryIdsDTO},
        pagination_dto::PaginationParamsDTO,
        post_dto::{CreatePostDTO, DeletePostIdsDTO, PostDTO},
        tag_dto::{CreateTagDTO, DeleteTagIdsDTO, TagDTO},
    },
    middlewares::error_middleware::Error,
    models::posts_model::PostsStatus,
};

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    components(schemas(
        Claims
    )),
    security(
        ("bearer_auth" = [])
    ),
    paths(
        crate::controllers::categories::create_category_controller::create_category_controller,
        crate::controllers::categories::get_all_categories_controller::get_all_categories_controller,
        crate::controllers::categories::get_category_by_id_controller::get_category_by_id_controller,
        crate::controllers::categories::delete_category_controller::delete_category_controller,
        crate::controllers::categories::update_category_controller::update_category_controller,
        crate::controllers::tags::create_tag_controller::create_tag_controller,
        crate::controllers::tags::delete_tag_controller::delete_tag_controller,
        crate::controllers::tags::get_tag_by_id_controller::get_tag_by_id_controller,
        crate::controllers::tags::get_all_tags_controller::get_all_tags_controller,
        crate::controllers::posts::create_post_controller::create_post_controller,
        crate::controllers::posts::get_all_posts_controller::get_all_posts_controller,
        crate::controllers::posts::create_post_controller::create_post_controller,
    ),
    components(schemas(Error, DeleteCategoryIdsDTO, CategoryDTO, CreateCategoryDTO,
        TagDTO, PostDTO, CreateTagDTO, DeleteTagIdsDTO, CreatePostDTO, DeletePostIdsDTO, PaginationParamsDTO, PostsStatus)),
    servers(
        (url = "/api/v1", description = "API v1")
    )
)]

pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
    tail: web::types::Path<String>,
    openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> web::HttpResponse {
    if tail.as_ref() == "swagger.json" {
        let spec = match ApiDoc::openapi().to_json() {
            Ok(json) => json,
            Err(_) => return web::HttpResponse::InternalServerError().finish(),
        };
        return web::HttpResponse::Ok()
            .content_type("application/json")
            .body(spec);
    }

    let conf = openapi_conf.as_ref().clone();
    match utoipa_swagger_ui::serve(&tail, conf.into()) {
        Ok(None) => web::HttpResponse::NotFound().finish(),
        Ok(Some(file)) => {
            let bytes = Bytes::from(file.bytes.to_vec());
            web::HttpResponse::Ok()
                .content_type(file.content_type)
                .body(bytes)
        }
        Err(_) => todo!(),
    }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    let swagger_config = Arc::new(
        utoipa_swagger_ui::Config::new(["/swagger/swagger.json"])
            .use_base_layout(),
    );
    config.service(
        web::scope("/swagger/")
            .state(swagger_config)
            .service(get_swagger),
    );
}
