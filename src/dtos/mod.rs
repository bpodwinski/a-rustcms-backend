use serde::Serialize;

pub mod category_dto;
pub mod post_dto;
pub mod tag_dto;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub error: String,
}
