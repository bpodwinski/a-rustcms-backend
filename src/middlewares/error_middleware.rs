use ntex::http::header;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<String>,
}

impl<S> Middleware<S> for Error {
    type Service = ErrorMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        ErrorMiddleware { service }
    }
}

pub struct ErrorMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for ErrorMiddleware<S>
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
        ctx.call(&self.service, req).await.map(|mut res| {
            let status = res.status();
            if status.is_client_error() || status.is_server_error() {
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json"),
                );
            }
            res
        })
    }
}
