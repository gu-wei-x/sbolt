mod welcome;
use rocket::Route;
use rocket::routes;

pub(crate) fn routes() -> Vec<Route> {
    routes![welcome::welcome]
}

use rocket::http::ContentType;
use rocket::http::Header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{self, Responder, Response};
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ResponseFormat {
    type Error = RuntimeError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let accept = request.headers().get("Accept").collect::<Vec<_>>();
        let preferences = (!accept.is_empty()).then(|| accept.join(","));
        match resolve_kind(preferences.as_deref()) {
            Ok(kind) => Outcome::Success(Self(Some(kind))),
            Err(RuntimeError::InvalidInput(..) | RuntimeError::NotFound(..)) => {
                Outcome::Success(Self(None))
            }
            Err(error) => {
                eprintln!("Response format resolution failed: {error}");
                Outcome::Error((Status::InternalServerError, error))
            }
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

impl<'r, 'o: 'r> Responder<'r, 'o> for TemplateResult {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        let mut response = match self.inner_result {
            RenderResult::Ok((output, kind)) => Response::build()
                .status(Status::Ok)
                .sized_body(output.len(), std::io::Cursor::new(output))
                .header(match kind {
                    Kind::KHTML => ContentType::HTML,
                    Kind::KJSON => ContentType::JSON,
                    Kind::KTEXT => ContentType::Plain,
                })
                .ok(),
            RenderResult::Err(e) => {
                eprintln!("Template render failed: {e}");
                let error_message = "Internal server error";
                Response::build()
                    .status(Status::InternalServerError)
                    .header(ContentType::Plain)
                    .sized_body(error_message.len(), std::io::Cursor::new(error_message))
                    .ok()
            }
        }?;
        response.set_header(Header::new("Vary", "Accept"));
        Ok(response)
    }
}

#[cfg(test)]
mod tests;
