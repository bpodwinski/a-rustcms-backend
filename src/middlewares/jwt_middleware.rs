use http::header::AUTHORIZATION;
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, Validation,
};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web::{self, HttpResponse, WebResponse};
use serde::{Deserialize, Serialize};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Middleware structure to validate JWT
pub struct JwtMiddleware;

impl<S> Middleware<S> for JwtMiddleware {
    type Service = JwtService<S>;

    fn create(&self, service: S) -> Self::Service {
        JwtService { service }
    }
}

pub struct JwtService<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for JwtService<S>
where
    S: Service<
        web::WebRequest<Err>,
        Response = web::WebResponse,
        Error = web::Error,
    >,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        // Extract token from the Authorization header
        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                // Expecting Bearer token
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    match validate_jwt(token) {
                        Ok(_) => {
                            // Token is valid, forward the request
                            return ctx.call(&self.service, req).await;
                        }
                        Err(e) => {
                            // Token invalid or expired
                            let response = HttpResponse::Unauthorized()
                                .body(format!("Invalid token: {:?}", e));
                            return Ok(WebResponse::new(
                                response,
                                req.into_parts().0,
                            ));
                        }
                    }
                }
            }
        }

        // If token is missing or invalid, respond with Unauthorized
        let response = HttpResponse::Unauthorized()
            .body("Authorization token is missing or invalid");
        Ok(WebResponse::new(response, req.into_parts().0))
    }
}

/// Validate the JWT token
fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let decoding_key =
        DecodingKey::from_secret("TO6NXrE32UpCDqz72B4v14ie8qd1IOdc".as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
