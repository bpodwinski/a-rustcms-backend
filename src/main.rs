use dotenv::dotenv;
use env_logger::Env;
use middlewares::error_middleware::Error;
use ntex::web::{App, HttpServer};
use ntex_cors::Cors;

mod config;
mod controllers;
mod db;
mod dtos;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod tests;
mod validators;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = db::init_pool(config::config::get_database_url())
        .await
        .expect("Failed to create pool");
    let cors_allowed_url = config::config::get_cors_allowed_url();
    let api_url = config::config::get_api_url();
    let api_port = config::config::get_api_port();

    HttpServer::new(move || {
        App::new()
            .wrap(Error {
                message: String::from("Middleware error"),
                backtrace: None,
            })
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
            .configure(handlers::openapi::ntex_config)
            .configure(routes::init)
    })
    .workers(1)
    .bind((api_url, api_port))?
    .run()
    .await
}
