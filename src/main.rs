use dotenv::dotenv;
use ntex::web::{App, HttpServer};
use ntex_cors::Cors;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool().await.expect("Failed to create pool");

    HttpServer::new(move || App::new()
        .wrap(
            Cors::new()
                .allowed_origin("http://127.0.0.1:3000")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
        .state(pool.clone())
        .configure(routes::init))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
