disguise::include_views!();
mod handlers;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", handlers::routes())
        .launch()
        .await?;
    Ok(())
}
