mod welcome;
pub(crate) use welcome::welcome;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::http::header::ContentType;
use actix_web::{FromRequest, dev::Payload, error, http::header};
use sbolt::resolve_kind;
use sbolt::types::error::RuntimeError;
use sbolt::types::result::RenderResult;
use sbolt::types::template::Kind;
use std::future::{Ready, ready};

pub(crate) struct ResponseFormat(Option<Kind>);

impl From<ResponseFormat> for Option<Kind> {
    fn from(format: ResponseFormat) -> Self {
        format.0
    }
}

impl FromRequest for ResponseFormat {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let accept = request
            .headers()
            .get_all(header::ACCEPT)
            .map(|value| value.to_str())
            .collect::<Result<Vec<_>, _>>();
        let Ok(accept) = accept else {
            return ready(Ok(Self(None)));
        };
        let preferences = (!accept.is_empty()).then(|| accept.join(","));
        ready(match resolve_kind(preferences.as_deref()) {
            Ok(kind) => Ok(Self(Some(kind))),
            Err(RuntimeError::InvalidInput(..) | RuntimeError::NotFound(..)) => Ok(Self(None)),
            Err(error) => Err(error::ErrorInternalServerError(error)),
        })
    }
}

pub(crate) struct TemplateResult {
    inner_result: RenderResult<(String, Kind)>,
}

impl From<RenderResult<(String, Kind)>> for TemplateResult {
    fn from(result: RenderResult<(String, Kind)>) -> Self {
        Self {
            inner_result: result,
        }
    }
}

impl Responder for TemplateResult {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = match self.inner_result {
            RenderResult::Ok((output, kind)) => HttpResponse::Ok()
                .content_type(match kind {
                    Kind::KHTML => ContentType::html(),
                    Kind::KJSON => ContentType::json(),
                    Kind::KTEXT => ContentType::plaintext(),
                })
                .body(output),
            RenderResult::Err(e) => HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("Render Error: {}", e)),
        };
        response.headers_mut().insert(
            actix_web::http::header::VARY,
            actix_web::http::header::HeaderValue::from_static("Accept"),
        );
        response
    }
}

#[cfg(test)]
mod tests;
