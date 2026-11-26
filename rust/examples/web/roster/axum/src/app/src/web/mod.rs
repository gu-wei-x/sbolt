use std::{path::PathBuf, sync::Arc};

use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

mod controllers;

pub use roster_data::create_pool;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: SqlitePool,
    pub(crate) csrf: Arc<str>,
}

pub fn create_router(pool: SqlitePool, assets: PathBuf) -> Router {
    Router::new()
        .route("/", get(controllers::home))
        .merge(controllers::company_routes())
        .merge(controllers::employee_routes())
        .nest_service("/assets", ServeDir::new(assets))
        .with_state(AppState {
            pool,
            csrf: Arc::from(uuid::Uuid::new_v4().to_string()),
        })
}
