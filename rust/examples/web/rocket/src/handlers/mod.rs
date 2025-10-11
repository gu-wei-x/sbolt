mod welcome;
use rocket::Route;
use rocket::routes;

pub(crate) fn routes() -> Vec<Route> {
    routes![welcome::welcome]
}

use sbolt::types::result::RenderResult;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

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

impl<'r, 'o: 'r> Responder<'r, 'o> for TemplateResult {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        match self.inner_result {
            RenderResult::Ok(output) => Response::build()
                .status(Status::Ok)
                .sized_body(output.len(), std::io::Cursor::new(output))
                .header(ContentType::HTML)
                .ok(),
            RenderResult::Err(e) => {
                let error_message = e.to_string();
                Response::build()
                    .status(Status::InternalServerError)
                    .sized_body(error_message.len(), std::io::Cursor::new(error_message))
                    .ok()
            }
        }
    }
}
