use std::{env, error::Error, path::PathBuf};

pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub assets_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let database_url = match env::var("DATABASE_URL") {
            Ok(value) => value,
            Err(env::VarError::NotPresent) => "sqlite://roster.db".into(),
            Err(error) => return Err(error.into()),
        };
        let bind_addr = match env::var("BIND_ADDR") {
            Ok(value) => value,
            Err(env::VarError::NotPresent) => "127.0.0.1:3000".into(),
            Err(error) => return Err(error.into()),
        };
        let assets_dir = match env::var_os("ROSTER_ASSETS_DIR") {
            Some(path) => PathBuf::from(path),
            None => env::current_exe()?
                .parent()
                .ok_or("Cannot determine the executable directory.")?
                .join("assets"),
        };
        if !std::fs::metadata(&assets_dir)?.is_dir() {
            return Err(format!("Asset path is not a directory: {}", assets_dir.display()).into());
        }
        Ok(Self {
            database_url,
            bind_addr,
            assets_dir,
        })
    }
}
