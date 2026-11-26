use std::sync::Arc;

use actix_web::web;
use sqlx::SqlitePool;

mod controllers;

pub use roster_data::create_pool;

pub struct AppState {
    pub(crate) pool: SqlitePool,
    pub(crate) csrf: Arc<str>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            csrf: Arc::from(uuid::Uuid::new_v4().to_string()),
        }
    }
}

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/", web::get().to(controllers::home))
        .configure(controllers::company_routes)
        .configure(controllers::employee_routes);
}
