mod welcome;
pub(crate) use welcome::welcome;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use sbolt::resolve_kind;
use sbolt::types::error::RuntimeError;
use sbolt::types::result::RenderResult;
use sbolt::types::template::Kind;

pub(crate) struct ResponseFormat(Option<Kind>);

impl From<ResponseFormat> for Option<Kind> {
    fn from(format: ResponseFormat) -> Self {
        format.0
    }
}

impl<S: Send + Sync> FromRequestParts<S> for ResponseFormat {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let accept = parts
            .headers
            .get_all(header::ACCEPT)
            .iter()
            .map(|value| value.to_str())
            .collect::<Result<Vec<_>, _>>();
        let Ok(accept) = accept else {
            return Ok(Self(None));
        };
        let preferences = (!accept.is_empty()).then(|| accept.join(","));
        match resolve_kind(preferences.as_deref()) {
            Ok(kind) => Ok(Self(Some(kind))),
            Err(RuntimeError::InvalidInput(..) | RuntimeError::NotFound(..)) => Ok(Self(None)),
            Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
        }
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

impl IntoResponse for TemplateResult {
    fn into_response(self) -> Response {
        let mut response = match self.inner_result {
            RenderResult::Ok((output, kind)) => (
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    match kind {
                        Kind::KHTML => "text/html; charset=utf-8",
                        Kind::KJSON => "application/json",
                        Kind::KTEXT => "text/plain; charset=utf-8",
                    },
                )],
                output,
            )
                .into_response(),
            RenderResult::Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        };
        response
            .headers_mut()
            .insert(header::VARY, axum::http::HeaderValue::from_static("Accept"));
        response
    }
}

#[cfg(test)]
mod tests;
