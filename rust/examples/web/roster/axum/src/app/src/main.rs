use std::error::Error;

use roster_config::Config;

mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Config {
        database_url,
        bind_addr,
        assets_dir,
    } = Config::from_env()?;
    let pool = web::create_pool(&database_url).await?;
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    println!("Roster listening at http://{}", listener.local_addr()?);
    axum::serve(listener, web::create_router(pool, assets_dir))
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                eprintln!("Cannot listen for Ctrl+C: {error}");
            }
        })
        .await?;
    Ok(())
}
