use sqlx::{FromRow, PgPool};

pub mod categories;
pub mod posts;
pub mod posts_categories;
pub mod tags;

pub struct QueryBuilder<'a, T> {
    pool: &'a PgPool,
    table: String,
    fields: Vec<String>,
    values: Vec<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    query_type: QueryType,
    _marker: std::marker::PhantomData<T>,
}

#[derive(PartialEq)]
pub enum QueryType {
    Select,
    Insert,
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    // Initialisation du QueryBuilder avec une pool de connexion
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

    // Définir la table
    pub fn table(mut self, table: &str) -> Self {
        self.table = table.to_string();
        self
    }

    // Définir les champs à sélectionner
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.fields = fields.iter().map(|&f| f.to_string()).collect();
        self
    }

    // Définir les valeurs pour une requête d'insertion
    pub fn values(mut self, values: &[&str]) -> Self {
        self.values = values.iter().map(|&v| v.to_string()).collect();
        self.query_type = QueryType::Insert;
        self
    }

    // Définir la limite de résultats
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    // Définir l'offset (le décalage)
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    // Construire et exécuter la requête
    pub async fn select(self) -> Result<Vec<T>, sqlx::Error> {
        // Construction de la requête
        let mut query =
            format!("SELECT {} FROM {}", self.fields.join(", "), self.table);

        // Ajouter LIMIT si défini
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Ajouter OFFSET si défini
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        // Exécution de la requête SQL et récupération des résultats
        let rows = sqlx::query_as::<_, T>(&query).fetch_all(self.pool).await?;

        Ok(rows)
    }

    // Construire et exécuter une requête d'insertion
    pub async fn insert(self) -> Result<T, sqlx::Error> {
        if self.query_type != QueryType::Insert {
            return Err(sqlx::Error::RowNotFound);
        }

        // Construire la requête d'insertion
        let fields_str = self.fields.join(", ");
        let placeholders: Vec<String> =
            (1..=self.values.len()).map(|i| format!("${}", i)).collect();
        let placeholders_str = placeholders.join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.table, fields_str, placeholders_str
        );

        // Exécuter la requête d'insertion
        let affected_rows = sqlx::query_as(&query)
            .bind(self.values)
            .fetch_one(self.pool)
            .await?;

        Ok(affected_rows)
    }
}
