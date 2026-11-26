use std::error::Error;

use actix_files::Files;
use actix_web::{App, HttpServer, web::Data};
use roster_config::Config;

mod web;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Config {
        database_url,
        bind_addr,
        assets_dir,
    } = Config::from_env()?;
    let pool = web::create_pool(&database_url).await?;
    let state = Data::new(web::AppState::new(pool));
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(web::configure)
            .service(Files::new("/assets", &assets_dir))
    })
    .bind(&bind_addr)?;
    for address in server.addrs() {
        println!("Roster listening at http://{address}");
    }
    server.run().await?;
    Ok(())
}
