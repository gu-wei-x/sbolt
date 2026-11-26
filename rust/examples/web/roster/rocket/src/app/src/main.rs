use std::{error::Error, net::SocketAddr};

use roster_config::Config;

mod web;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Config {
        database_url,
        bind_addr,
        assets_dir,
    } = Config::from_env()?;
    let bind: SocketAddr = bind_addr.parse()?;
    let pool = web::create_pool(&database_url).await?;
    web::create_router(pool, assets_dir)
        .configure(rocket::Config {
            address: bind.ip(),
            port: bind.port(),
            ..rocket::Config::default()
        })
        .launch()
        .await?;
    Ok(())
}
