use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use ntex::http;
use ntex::web::guard::Guard;

use crate::config::config::get_secret_key;
use crate::dtos::auth_dtos::ClaimsDTO;

pub struct JwtGuard;

impl Guard for JwtGuard {
    fn check(&self, req: &http::RequestHead) -> bool {
        if let Some(auth_header) =
            req.headers().get(http::header::AUTHORIZATION)
        {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];

                    let decoding_key =
                        DecodingKey::from_secret(get_secret_key().as_ref());
                    let validation = Validation::new(Algorithm::HS256);

                    if decode::<ClaimsDTO>(token, &decoding_key, &validation)
                        .is_ok()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}
