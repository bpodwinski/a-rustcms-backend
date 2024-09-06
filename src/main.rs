use dotenv::dotenv;
use ntex::web::{App, HttpServer};
use ntex_cors::Cors;

mod config;
mod controllers;
mod db;
mod dto;
mod models;
mod validators;

mod repositories;
mod routes;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool().await.expect("Failed to create pool");
    let cors_allowed_url = crate::config::config::get_cors_allowed_url();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&cors_allowed_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .state(pool.clone())
            .configure(routes::init)
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
