use database::HasArguments;
use postgres::PgRow;
use query::{Query, QueryAs};
use sqlx::*;

pub mod categories_repository;
pub mod posts_categories_repository;
pub mod posts_repository;
pub mod tags_repository;

/// Enum to represent different types of bindable values for SQL queries
#[derive(Clone)]
enum Bind {
    Int(i32),
    Text(String),
    Null,
}

impl Bind {
    /// Binds a value to a `Query` or `QueryAs` type.
    pub fn bind_to_query<'q, DB, Q>(self, query: Q) -> Q
    where
        DB: Database,
        Q: BindableQuery<'q, DB>,
        i32: Encode<'q, DB> + Type<DB>,
        String: Encode<'q, DB> + Type<DB>,
        Option<i32>: Encode<'q, DB> + Type<DB>,
    {
        match self {
            Bind::Int(val) => query.bind_value(val),
            Bind::Text(val) => query.bind_value(val),
            Bind::Null => query.bind_value(None::<i32>),
        }
    }
}

/// A helper trait to generalize binding for both `Query` and `QueryAs`.
pub trait BindableQuery<'q, DB: Database>: Sized {
    fn bind_value<T>(self, value: T) -> Self
    where
        T: 'q + Send + Encode<'q, DB> + Type<DB>;
}

/// Implement `BindableQuery` for `Query`.
impl<'q, DB> BindableQuery<'q, DB>
    for Query<'q, DB, <DB as HasArguments<'q>>::Arguments>
where
    DB: Database,
{
    fn bind_value<T>(self, value: T) -> Self
    where
        T: 'q + Send + Encode<'q, DB> + Type<DB>,
    {
        self.bind(value)
    }
}

/// Implement `BindableQuery` for `QueryAs`.
impl<'q, DB, O> BindableQuery<'q, DB>
    for QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
where
    DB: Database,
{
    fn bind_value<T>(self, value: T) -> Self
    where
        T: 'q + Send + Encode<'q, DB> + Type<DB>,
    {
        self.bind(value)
    }
}

/// Struct to build and execute dynamic SQL queries
struct QueryBuilder<'a, T> {
    pool: &'a PgPool,
    table: String,
    fields: Vec<String>,
    values: Vec<Bind>,
    limit: Option<i64>,
    offset: Option<i64>,
    query_type: QueryType,
    _marker: std::marker::PhantomData<T>,
}

/// Enum to differentiate between query types: Select and Insert
#[derive(PartialEq)]
enum QueryType {
    Select,
    Insert,
    Delete,
    Update,
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    /// Initializes a new `QueryBuilder` with a given connection pool.
    ///
    /// # Arguments
    /// * `pool` - A reference to the connection pool (`PgPool`) used to execute queries.
    ///
    /// # Returns
    /// Returns a new instance of `QueryBuilder`.
    fn new(pool: &'a PgPool) -> Self {
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
    fn table(mut self, table: &str) -> Self {
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
    fn fields(mut self, fields: &[&str]) -> Self {
        self.fields = fields.iter().map(|&f| f.to_string()).collect();
        self
    }

    /// Sets the values for an insert query.
    ///
    /// # Arguments
    /// * `values` - A vector of `Bind` representing the values to be inserted.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` with the values set.
    fn values(mut self, values: Vec<Bind>) -> Self {
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
    fn limit(mut self, limit: i64) -> Self {
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
    fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds and executes a SELECT query, with the option to return either one or multiple rows.
    ///
    /// # Arguments
    /// * `id_field` - An optional field representing the ID (e.g., `category_id`).
    /// * `id_value` - An optional value of the ID to search for.
    /// * `single_result` - A boolean indicating whether to expect a single result or multiple results.
    ///
    /// # Returns
    /// Returns a `Result` containing either a single item (`QueryResult::Single(T)`) or multiple items (`QueryResult::Multiple(Vec<T>)`).
    async fn select(
        self,
        id_field: Option<&str>,
        id_value: Option<&Bind>,
    ) -> Result<Vec<T>, Error> {
        let mut query =
            format!("SELECT {} FROM {}", self.fields.join(", "), self.table);

        // Add WHERE clause if an ID filter is provided
        if let Some(id_field) = id_field {
            query.push_str(&format!(" WHERE {} = $1", id_field));
        }

        // Add LIMIT if defined
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Add OFFSET if defined
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let mut sql_query = query_as::<_, T>(&query);

        if let Some(bind_value) = id_value {
            sql_query = bind_value.clone().bind_to_query(sql_query);
        }

        // Execute the query and fetch all results
        let rows = sql_query.fetch_all(self.pool).await?;
        Ok(rows)
    }

    /// Builds and executes a SELECT query, returning a single row.
    async fn select_one(
        self,
        id_field: Option<&str>,
        id_value: Option<&Bind>,
    ) -> Result<T, Error> {
        let mut query =
            format!("SELECT {} FROM {}", self.fields.join(", "), self.table);

        // Add WHERE clause if an ID filter is provided
        if let Some(id_field) = id_field {
            query.push_str(&format!(" WHERE {} = $1", id_field));
        }

        // Add LIMIT 1 to ensure only one result is returned
        query.push_str(" LIMIT 1");

        let mut sql_query = query_as::<_, T>(&query);

        if let Some(id_value) = id_value {
            sql_query = id_value.clone().bind_to_query(sql_query);
        }

        let row = sql_query.fetch_one(self.pool).await?;
        Ok(row)
    }

    /// Builds and executes an INSERT query.
    ///
    /// # Returns
    /// Returns a `Result` containing the inserted row, or an error.
    async fn insert(self) -> Result<T, Error> {
        let mut tx = self.pool.begin().await?;

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

        for value in self.values {
            sql_query = value.bind_to_query(sql_query);
        }

        let result = sql_query.fetch_one(&mut *tx).await?;

        tx.commit().await?;

        Ok(result)
    }

    /// Builds and executes an UPDATE query based on a condition.
    ///
    /// # Arguments
    /// * `condition_field` - The field to apply the condition to (e.g., `category_id`).
    /// * `condition_value` - The value to bind to the condition.
    ///
    /// # Returns
    /// Returns a `Result` containing the number of rows affected, or an error.
    async fn update(
        self,
        condition_field: &str,
        condition_value: Bind,
    ) -> Result<T, Error> {
        let mut tx = self.pool.begin().await?;

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
            self.fields.len() + 1
        );

        let mut sql_query = query_as(&query);

        for value in &self.values {
            sql_query = value.clone().bind_to_query(sql_query);
        }

        sql_query = condition_value.bind_to_query(sql_query);

        let result = sql_query.fetch_one(&mut *tx).await?;

        tx.commit().await?;

        Ok(result)
    }

    /// Builds and executes a DELETE query that deletes multiple rows based on a list of IDs.
    ///
    /// # Arguments
    /// * `column` - The column to apply the `WHERE` condition (e.g., "id").
    /// * `ids` - A list of IDs to be deleted.
    ///
    /// # Returns
    /// Returns a `Result` containing the list of deleted IDs.
    async fn delete(
        self,
        column: &str,
        ids: Vec<i32>,
    ) -> Result<Vec<i32>, Error> {
        let mut tx = self.pool.begin().await?;

        let query = format!(
            "DELETE FROM {} WHERE {} = ANY($1::int[]) RETURNING {}",
            self.table, column, column
        );

        let mut sql_query = query_as::<_, (i32,)>(&query);

        sql_query = Bind::Int(ids.len() as i32).bind_to_query(sql_query);

        let rows = sql_query.bind(&ids).fetch_all(&mut *tx).await?;

        tx.commit().await?;

        let deleted_ids: Vec<i32> = rows.into_iter().map(|(id,)| id).collect();
        Ok(deleted_ids)
    }

    /// Builds and executes a COUNT query to count the number of rows.
    ///
    /// # Returns
    /// Returns a `Result` containing the count of rows.
    async fn count(self) -> Result<i64, Error> {
        let query = format!("SELECT COUNT(*) FROM {}", self.table);

        let row: (i64,) = sqlx::query_as(&query).fetch_one(self.pool).await?;

        Ok(row.0)
    }
}
