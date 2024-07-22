use crate::models::post::{NewPost, Post};
use ntex::web::types::Path;
use ntex::web::{self, HttpResponse};
use sqlx::PgPool;

/// Fetch all posts from the database.
///
/// This function queries the `posts` table in the database and returns all the
/// rows as a JSON array.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
///
/// # Returns
///
/// A `HttpResponse` containing a JSON array of all posts or an internal server error.
pub async fn get_posts(pool: web::types::State<PgPool>) -> HttpResponse {
    let posts = match sqlx::query_as!(Post, "SELECT id, title, content, author_id FROM posts")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(posts) => posts,
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(&posts)
}

/// Fetch a specific post by its ID.
///
/// This function queries the `posts` table in the database for a row with the
/// specified ID and returns it as JSON.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to fetch.
///
/// # Returns
///
/// A `HttpResponse` containing the post as JSON or an internal server error if the post is not found.
pub async fn get_post_by_id(pool: web::types::State<PgPool>, post_id: Path<i32>) -> HttpResponse {
    let post_id = post_id.into_inner();

    match sqlx::query_as!(
        Post,
        "SELECT id, title, content, author_id FROM posts WHERE id = $1",
        post_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(post) => HttpResponse::Ok().json(&post),
        Err(e) => {
            eprintln!("Error fetching post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Create a new post in the database.
///
/// This function inserts a new row into the `posts` table in the database with the
/// provided title, content, and author ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `new_post` - A JSON object containing the new post data.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation. If successful, it returns a `Created` status.
pub async fn create_post(
    pool: web::types::State<PgPool>,
    new_post: web::types::Json<NewPost>,
) -> HttpResponse {
    let new_post = new_post.into_inner();

    match sqlx::query!(
        "INSERT INTO posts (title, content, author_id) VALUES ($1, $2, $3)",
        new_post.title,
        new_post.content,
        new_post.author_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Failed to create post: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Update a specific post by its ID.
///
/// This function updates a row in the `posts` table in the database with the
/// specified ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to update.
/// * `updated_post` - A JSON object containing the updated post data.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation. If successful, it returns a `No Content` status.
pub async fn update_post_by_id(
    pool: web::types::State<PgPool>,
    post_id: Path<i32>,
    updated_post: web::types::Json<NewPost>,
) -> HttpResponse {
    let post_id = post_id.into_inner();
    let updated_post = updated_post.into_inner();

    match sqlx::query!(
        "UPDATE posts SET title = $1, content = $2, author_id = $3 WHERE id = $4",
        updated_post.title,
        updated_post.content,
        updated_post.author_id,
        post_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error updating post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete a specific post by its ID.
///
/// This function deletes a row from the `posts` table in the database with the
/// specified ID.
///
/// # Arguments
///
/// * `pool` - A `PgPool` instance provided by `ntex` for database access.
/// * `post_id` - The ID of the post to delete.
///
/// # Returns
///
/// A `HttpResponse` indicating the result of the operation. If successful, it returns a `No Content` status.
pub async fn delete_post_by_id(
    pool: web::types::State<PgPool>,
    post_id: Path<i32>,
) -> HttpResponse {
    let post_id = post_id.into_inner();

    match sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting post by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
