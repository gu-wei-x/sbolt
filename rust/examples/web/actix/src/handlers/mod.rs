mod welcome;
pub(crate) use welcome::welcome;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::http::header::ContentType;
use disguise::types::result::RenderResult;

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

impl Responder for TemplateResult {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.inner_result {
            RenderResult::Ok(output) => HttpResponse::Ok().body(output),
            RenderResult::Err(e) => HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("Render Error: {}", e)),
        }
    }
}
