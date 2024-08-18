use ntex::web::{self, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::models::posts::posts_categories_table_model::PostsCategories;
use crate::models::posts::posts_table_model::Post;
use crate::services::posts::create_post::create_post_service;

#[derive(Serialize)]
struct PostWithCategories {
    post: Post,
    categories: Vec<PostsCategories>,
}

#[derive(Deserialize)]
struct CreatePostRequest {
    post: Post,
    categories_ids: Vec<i32>,
}

/// Create a new post in the database.
///
/// This function inserts a new row into the `posts` table in the database
/// with the provided title, content, and author ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` pour accéder à la base de données.
/// * `request` - Un objet JSON contenant les données du post et les IDs des catégories associées.
///
/// # Returns
///
/// Une `HttpResponse` indiquant le résultat de l'opération.
/// Si elle réussit, elle retourne un statut `Created` avec le post créé et ses catégories.
#[web::post("/posts")]
pub async fn create_post_controller(
    pool: web::types::State<PgPool>,
    request: web::types::Json<CreatePostRequest>,
) -> HttpResponse {
    let CreatePostRequest {
        post,
        categories_ids,
    } = request.into_inner();

    if let Err(errors) = post.validate() {
        return HttpResponse::BadRequest().json(&errors);
    }

    match create_post_service(pool.get_ref(), post, categories_ids).await {
        Ok((post, posts_categories)) => {
            let response = PostWithCategories {
                post,
                categories: posts_categories,
            };
            HttpResponse::Created().json(&response)
        }
        Err(err) => {
            eprintln!("Failed to create post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
