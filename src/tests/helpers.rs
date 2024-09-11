#[cfg(test)]
pub mod setup {
    use sqlx::{Pool, Postgres};

    use crate::config::config;
    use crate::db;

    pub async fn setup_test_db() -> Pool<Postgres> {
        dotenv::from_filename(".env.test").ok();
        let database_url = config::get_database_url();

        let pool = db::init_pool(database_url)
            .await
            .expect("Failed to create pool");

        pool
    }
}
