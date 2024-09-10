#[cfg(test)]
pub mod setup {
    use sqlx::migrate::MigrateDatabase;
    use sqlx::{Pool, Postgres};

    use crate::config::config;
    use crate::db;

    pub async fn setup_test_db() -> Pool<Postgres> {
        dotenv::from_filename(".env.test").ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        if !Postgres::database_exists(&database_url).await.unwrap() {
            Postgres::create_database(&database_url).await.unwrap();
        }

        let pool = db::init_pool(config::get_database_url())
            .await
            .expect("Failed to create pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }
}
