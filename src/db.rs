use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = crate::config::config::get_database_url();
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
}
