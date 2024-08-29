use serde::Serialize;

pub mod category_dto;
pub mod post_dto;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub error: String,
}
