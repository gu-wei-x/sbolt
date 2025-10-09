disguise::include_views!();
mod handlers;
use rocket::Config;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config {
        address: "127.0.0.1".parse().unwrap(),
        port: 8000,
        ..Config::default()
    };
    rocket::build()
        .configure(config)
        .mount("/", handlers::routes())
        .launch()
        .await?;
    Ok(())
}
