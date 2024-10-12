use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenDTO {
    pub token: String,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClaimsDTO {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct LoginRequestDTO {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Email must be between 1 and 100 characters"
    ))]
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "Password must be at least 6 characters long"
    ))]
    pub password: String,
}
