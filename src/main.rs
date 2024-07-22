use dotenv::dotenv;
use ntex::web::{App, HttpServer};

mod config;
mod db;
mod handlers;
mod models;
mod routes;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool().await.expect("Failed to create pool");

    HttpServer::new(move || App::new().state(pool.clone()).configure(routes::init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
