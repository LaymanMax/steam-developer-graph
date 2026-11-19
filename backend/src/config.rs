use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_path: PathBuf,
    pub dataset_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3001);
        let database_path = env::var("DATABASE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./data/steam_games.db"));
        let dataset_path = env::var("DATASET_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./data/steam_games.parquet"));

        Self {
            host,
            port,
            database_path,
            dataset_path,
        }
    }
}
