pub mod migrations;
pub mod queries;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::Path;
use tracing::info;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db(db_path: &Path) -> Result<DbPool, Box<dyn std::error::Error>> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    info!("Initializing database at {:?}", db_path);

    // Run migrations first on a single direct connection
    {
        let mut conn = Connection::open(db_path)?;
        migrations::run_migrations(&mut conn)?;
    }

    let manager = SqliteConnectionManager::file(db_path).with_init(|c| {
        c.execute_batch(
            "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA foreign_keys = ON;
                 PRAGMA cache_size = -64000;
                 PRAGMA temp_store = MEMORY;
                 PRAGMA busy_timeout = 10000;
                ",
        )
    });

    let pool = Pool::builder().max_size(16).build(manager)?;

    Ok(pool)
}
