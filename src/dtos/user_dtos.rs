use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

use crate::models::users_models::UserModel;

/// Batch deletion of user
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteUserIdsDTO {
    pub ids: Vec<i32>,
}

/// Creating a user
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDTO {
    pub login: String,
    pub password: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub url: Option<String>,
    pub active: bool,
}

/// Converts `CreateUserDTO` to `UserModel`
impl TryFrom<CreateUserDTO> for UserModel {
    type Error = ValidationErrors;

    fn try_from(dto: CreateUserDTO) -> Result<Self, Self::Error> {
        let user = UserModel {
            id: None,
            login: dto.login,
            password: dto.password,
            email: dto.email,
            firstname: dto.firstname,
            lastname: dto.lastname,
            url: dto.url,
            active: dto.active,
            date_created: None,
        };

        user.validate()?;
        Ok(user)
    }
}

/// Full user data
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserDTO {
    pub id: Option<i32>,
    pub login: String,
    pub password: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub url: Option<String>,
    pub active: bool,
    #[schema(value_type = String, format = "date-time", example = "2022-01-01T00:00:00")]
    pub date_created: Option<NaiveDateTime>,
}

/// Converts `UserModel` to `UserDTO`
impl From<UserModel> for UserDTO {
    fn from(user: UserModel) -> Self {
        UserDTO {
            id: user.id,
            login: user.login,
            password: user.password,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
            url: user.url,
            active: user.active,
            date_created: user.date_created,
        }
    }
}

/// Converts `UserDTO` to `UserModel`
impl TryFrom<UserDTO> for UserModel {
    type Error = ValidationErrors;

    fn try_from(dto: UserDTO) -> Result<Self, Self::Error> {
        let user = UserModel {
            id: dto.id,
            login: dto.login,
            password: dto.password,
            email: dto.email,
            firstname: dto.firstname,
            lastname: dto.lastname,
            url: dto.url,
            active: dto.active,
            date_created: dto.date_created,
        };

        user.validate()?;
        Ok(user)
    }
}
