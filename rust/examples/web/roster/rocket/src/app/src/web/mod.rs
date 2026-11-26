use std::{path::PathBuf, sync::Arc};

use rocket::{Build, Rocket, fs::FileServer};
use sqlx::SqlitePool;

mod controllers;

pub use roster_data::create_pool;

pub(crate) struct AppState {
    pub(crate) pool: SqlitePool,
    pub(crate) csrf: Arc<str>,
}

pub fn create_router(pool: SqlitePool, assets: PathBuf) -> Rocket<Build> {
    rocket::build()
        .manage(AppState {
            pool,
            csrf: Arc::from(uuid::Uuid::new_v4().to_string()),
        })
        .mount("/", controllers::home_routes())
        .mount("/", controllers::company_routes())
        .mount("/", controllers::employee_routes())
        .mount("/assets", FileServer::from(assets))
}
