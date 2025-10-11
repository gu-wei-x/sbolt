sbolt::include_views!();
mod handlers;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(handlers::welcome)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
