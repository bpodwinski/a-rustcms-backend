use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_pool(
    database_url: String,
) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}
