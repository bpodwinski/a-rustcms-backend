use anyhow::{anyhow, Result};
use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

use crate::{
    config::config::get_secret_key,
    dtos::auth_dtos::{ClaimsDTO, TokenDTO},
    repositories::users_repository::select_user_by_email,
};

pub async fn login_service(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<Option<TokenDTO>> {
    let user = match select_user_by_email(pool, email).await {
        Ok(user) => user,
        Err(e) => {
            return Err(anyhow!(
                "Failed to find user with email {}: {}",
                email,
                e
            ));
        }
    };

    let argon2 = Argon2::default();
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(e) => return Err(anyhow!("Failed to parse password hash: {}", e)),
    };

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            let claims = ClaimsDTO {
                sub: user
                    .id
                    .expect("L'utilisateur doit avoir un ID")
                    .to_string(),
                exp: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::hours(24))
                    .expect("Erreur lors de la création du token")
                    .timestamp() as usize,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(get_secret_key().as_ref()),
            )
            .map_err(|e| {
                anyhow!("Erreur lors de la génération du jeton: {}", e)
            })?;

            let token_dto = TokenDTO {
                token,
                token_type: "Bearer".to_string(),
                expires_in: 3600,
            };

            Ok(Some(token_dto))
        }
        Err(_) => Ok(None),
    }
}
