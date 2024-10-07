use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;
use std::marker::PhantomData;

pub mod categories_service;
pub mod posts_services;
pub mod tags_service;

/// A flexible service builder that handles both single model and Vec<Model> operations.
pub struct ServiceBuilder<'a, DTOIn, DTOOut, Model> {
    pool: &'a PgPool, // Le pool est une référence avec une durée de vie 'a
    input_dto: Option<DTOIn>,
    model: Option<Model>,
    model_vec: Option<Vec<Model>>,
    output_dto: Option<DTOOut>,
    output_dto_vec: Option<Vec<DTOOut>>,
    operation_mode: OperationMode,
    _marker: PhantomData<Model>,
}

/// Enum to indicate if we're working with a single model or a collection of models
pub enum OperationMode {
    Single,
    Collection,
}

#[derive(Serialize)]
pub enum OutputResult<DTOOut> {
    Single(DTOOut),
    Collection(Vec<DTOOut>),
}

impl<'a, DTOIn, DTOOut, Model> ServiceBuilder<'a, DTOIn, DTOOut, Model> {
    /// Create a new ServiceBuilder with the given operation mode
    pub fn new(pool: &'a PgPool, operation_mode: OperationMode) -> Self {
        ServiceBuilder {
            pool, // On utilise la référence directement ici
            input_dto: None,
            model: None,
            model_vec: None,
            output_dto: None,
            output_dto_vec: None,
            operation_mode,
            _marker: PhantomData,
        }
    }

    /// Appel de repository générique (un seul ou plusieurs modèles)
    pub async fn repository<F, Fut>(mut self, repo_fn: F) -> Result<Self>
    where
        F: FnOnce(&'a PgPool) -> Fut, // Le pool est passé par référence
        Fut: std::future::Future<Output = Result<Vec<Model>>>, // Générique pour renvoyer plusieurs modèles
    {
        match self.operation_mode {
            OperationMode::Single => {
                let result_model = repo_fn(self.pool).await?;
                self.model = Some(result_model.into_iter().next().unwrap()); // Extraire un seul modèle
            }
            OperationMode::Collection => {
                let result_models = repo_fn(self.pool).await?;
                self.model_vec = Some(result_models); // Gérer la collection
            }
        }
        Ok(self)
    }

    /// Convertir les modèles en DTOs
    pub fn output_dto(mut self) -> Self
    where
        DTOOut: From<Model>,
    {
        match self.operation_mode {
            OperationMode::Collection => {
                if let Some(models) = self.model_vec.take() {
                    let dto_vec: Vec<DTOOut> =
                        models.into_iter().map(DTOOut::from).collect();
                    self.output_dto_vec = Some(dto_vec);
                }
            }
            OperationMode::Single => {
                if let Some(model) = self.model.take() {
                    self.output_dto = Some(DTOOut::from(model));
                }
            }
        }
        self
    }

    /// Build the final result, which can be a single DTO or a collection
    pub fn build(self) -> OutputResult<DTOOut> {
        match self.operation_mode {
            OperationMode::Collection => OutputResult::Collection(
                self.output_dto_vec
                    .expect("Expected output DTO vector to be available"),
            ),
            OperationMode::Single => OutputResult::Single(
                self.output_dto
                    .expect("Expected output DTO to be available"),
            ),
        }
    }
}
