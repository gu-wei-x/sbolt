mod welcome;
pub(crate) use welcome::welcome;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sbolt::types::result::RenderResult;

pub(crate) struct TemplateResult {
    inner_result: RenderResult<String>,
}

impl From<RenderResult<String>> for TemplateResult {
    fn from(result: RenderResult<String>) -> Self {
        TemplateResult {
            inner_result: result,
        }
    }
}

impl IntoResponse for TemplateResult {
    fn into_response(self) -> Response {
        match self.inner_result {
            RenderResult::Ok(output) => (StatusCode::OK, Html(output)).into_response(),
            RenderResult::Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
