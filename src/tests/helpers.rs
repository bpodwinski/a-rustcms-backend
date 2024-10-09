#[cfg(test)]
pub mod setup {
    use std::fmt::Write;

    use sqlx::{Pool, Postgres};

    use crate::config::config;
    use crate::db;

    /// <summary>
    /// Set up a test database connection pool.
    /// </summary>
    /// <returns>A `Pool<Postgres>` representing the connection pool to the test database.</returns>
    pub async fn setup_test_db() -> Pool<Postgres> {
        // <remarks>
        // Loads environment variables from `.env.test` to configure the database.
        // </remarks>
        dotenv::from_filename(".env.test").ok();
        let database_url = config::get_database_url();

        // <remarks>
        // Initializes the database connection pool based on the test database URL.
        // </remarks>
        let pool = db::init_pool(database_url)
            .await
            .expect("Failed to create pool");

        pool
    }

    /// <summary>
    /// Clean test data from the specified table where the field matches a value.
    /// </summary>
    /// <param name="pool">A reference to the database connection pool.</param>
    /// <param name="table_name">The name of the table to delete data from.</param>
    /// <param name="field_name">The name of the field to match for deletion.</param>
    /// <param name="field_value">The value of the field to match for deletion.</param>
    /// <returns>A `Result<(), sqlx::Error>` indicating success or failure of the operation.</returns>
    pub async fn clean_data_test(
        pool: &Pool<Postgres>,
        table_name: &str,
        field_name: &str,
        field_value: &str,
    ) -> Result<(), sqlx::Error> {
        // <remarks>
        // Ensure the table name contains only valid alphanumeric characters or underscores.
        // </remarks>
        if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(sqlx::Error::Protocol("Invalid table name".into()));
        }

        // <remarks>
        // Ensure the field name contains only valid alphanumeric characters or underscores.
        // </remarks>
        if !field_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(sqlx::Error::Protocol("Invalid field name".into()));
        }

        // <remarks>
        // Dynamically create the SQL query string with the specified table and field names.
        // </remarks>
        let mut query = String::new();
        write!(
            &mut query,
            "DELETE FROM {} WHERE {} = $1",
            table_name, field_name
        )
        .unwrap();

        // <remarks>
        // Execute the query and bind the field value safely to prevent SQL injection.
        // </remarks>
        sqlx::query(&query)
            .bind(field_value)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|err| {
                eprintln!(
                    "Failed to delete from {} where {} = {}: {}",
                    table_name, field_name, field_value, err
                );
                err
            })
    }
}
