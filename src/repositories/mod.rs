use postgres::PgRow;
use sqlx::*;

pub mod categories;
pub mod posts;
pub mod posts_categories;
pub mod tags;

/// Enum to represent different types of bindable values for SQL queries
pub enum BindValue {
    Int(i32),
    Text(String),
    Null,
}

/// Struct to build and execute dynamic SQL queries
pub struct QueryBuilder<'a, T> {
    pool: &'a PgPool,
    table: String,
    fields: Vec<String>,
    values: Vec<BindValue>,
    limit: Option<i64>,
    offset: Option<i64>,
    query_type: QueryType,
    _marker: std::marker::PhantomData<T>,
}

/// Enum to differentiate between query types: Select and Insert
#[derive(PartialEq)]
pub enum QueryType {
    Select,
    Insert,
    Update,
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    /// Initializes a new `QueryBuilder` with a given connection pool.
    ///
    /// # Arguments
    /// * `pool` - A reference to the connection pool (`PgPool`) used to execute queries.
    ///
    /// # Returns
    /// Returns a new instance of `QueryBuilder`.
    pub fn new(pool: &'a PgPool) -> Self {
        QueryBuilder {
            pool,
            table: String::new(),
            fields: vec![],
            values: vec![],
            limit: None,
            offset: None,
            query_type: QueryType::Select,
            _marker: std::marker::PhantomData,
        }
    }

    /// Sets the table for the query.
    ///
    /// # Arguments
    /// * `table` - The name of the table.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the table set.
    pub fn table(mut self, table: &str) -> Self {
        self.table = table.to_string();
        self
    }

    /// Sets the fields to be used in the query.
    ///
    /// # Arguments
    /// * `fields` - A list of field names to be included in the query.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the fields set.
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.fields = fields.iter().map(|&f| f.to_string()).collect();
        self
    }

    /// Sets the values for an insert query.
    ///
    /// # Arguments
    /// * `values` - A vector of `BindValue` representing the values to be inserted.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the values set.
    pub fn values(mut self, values: Vec<BindValue>) -> Self {
        self.values = values;
        self.query_type = QueryType::Insert;
        self
    }

    /// Sets a limit on the number of rows returned by the query.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the limit set.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets an offset to skip a certain number of rows in the result set.
    ///
    /// # Arguments
    /// * `offset` - The number of rows to skip.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the offset set.
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds and executes a SELECT query.
    ///
    /// # Returns
    /// Returns a `Result` containing a vector of the result rows, or an error.
    pub async fn select(self) -> Result<Vec<T>, Error> {
        // Build the query
        let mut query =
            format!("SELECT {} FROM {}", self.fields.join(", "), self.table);

        // Add LIMIT if defined
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Add OFFSET if defined
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let rows = query_as::<_, T>(&query).fetch_all(self.pool).await?;

        Ok(rows)
    }

    /// Builds and executes an INSERT query.
    ///
    /// # Returns
    /// Returns a `Result` containing the inserted row, or an error.
    pub async fn insert(self) -> Result<T, Error> {
        if self.query_type != QueryType::Insert {
            return Err(Error::RowNotFound);
        }

        let fields_str = self.fields.join(", ");
        let placeholders_str = (1..=self.values.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.table, fields_str, placeholders_str
        );

        let mut sql_query = query_as::<_, T>(&query);

        // Bind each value according to its type
        for value in &self.values {
            match value {
                BindValue::Int(val) => {
                    sql_query = sql_query.bind(val);
                }
                BindValue::Text(val) => {
                    sql_query = sql_query.bind(val);
                }
                BindValue::Null => {
                    sql_query = sql_query.bind(None::<i32>);
                }
            }
        }

        let row = sql_query.fetch_one(self.pool).await?;

        Ok(row)
    }

    /// Builds and executes a DELETE query based on a condition.
    ///
    /// # Arguments
    /// * `condition_field` - The field to apply the condition to (e.g., `category_id`).
    /// * `condition_value` - The value to bind to the condition.
    ///
    /// # Returns
    /// Returns a `Result` containing the number of rows affected, or an error.
    pub async fn delete(
        self,
        condition_field: &str,
        condition_value: BindValue,
    ) -> Result<u64, Error> {
        let query = format!(
            "DELETE FROM {} WHERE {} = $1",
            self.table, condition_field
        );

        let mut sql_query = sqlx::query(&query);

        // Bind each value according to its type
        match condition_value {
            BindValue::Int(val) => {
                sql_query = sql_query.bind(val);
            }
            BindValue::Text(val) => {
                sql_query = sql_query.bind(val);
            }
            BindValue::Null => {
                sql_query = sql_query.bind(None::<i32>);
            }
        }

        let result = sql_query.execute(self.pool).await?;

        Ok(result.rows_affected())
    }

    /// Builds and executes an UPDATE query based on a condition.
    ///
    /// # Arguments
    /// * `condition_field` - The field to apply the condition to (e.g., `category_id`).
    /// * `condition_value` - The value to bind to the condition.
    ///
    /// # Returns
    /// Returns a `Result` containing the number of rows affected, or an error.
    pub async fn update(
        self,
        condition_field: &str,
        condition_value: BindValue,
    ) -> Result<u64, Error> {
        if self.query_type != QueryType::Update {
            return Err(Error::RowNotFound);
        }

        // Build the query: Set fields dynamically
        let update_fields_str = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| format!("{} = ${}", field, i + 1))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            self.table,
            update_fields_str,
            condition_field,
            self.fields.len() + 1 // Placeholder for the condition
        );

        let mut sql_query = sqlx::query(&query);

        // Bind each value according to its type
        for value in &self.values {
            match value {
                BindValue::Int(val) => {
                    sql_query = sql_query.bind(val);
                }
                BindValue::Text(val) => {
                    sql_query = sql_query.bind(val);
                }
                BindValue::Null => {
                    sql_query = sql_query.bind(None::<i32>);
                }
            }
        }

        // Bind the condition value
        match condition_value {
            BindValue::Int(val) => {
                sql_query = sql_query.bind(val);
            }
            BindValue::Text(val) => {
                sql_query = sql_query.bind(val);
            }
            BindValue::Null => {
                sql_query = sql_query.bind(None::<i32>);
            }
        }

        let result = sql_query.execute(self.pool).await?;

        Ok(result.rows_affected())
    }
}
