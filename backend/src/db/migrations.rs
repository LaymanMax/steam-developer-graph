use rusqlite::{Connection, Result};
use tracing::info;

pub fn run_migrations(conn: &mut Connection) -> Result<()> {
    info!("Running database migrations...");

    // Enable WAL mode, foreign keys, and performant pragmas
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;
         PRAGMA cache_size = -64000; -- 64MB cache
         PRAGMA temp_store = MEMORY;
        ",
    )?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );
        ",
    )?;

    let current_version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < 1 {
        let tx = conn.transaction()?;
        tx.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS ingestion_runs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_type TEXT NOT NULL,
                source_url TEXT,
                file_path TEXT,
                file_size_bytes INTEGER,
                file_checksum TEXT,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                status TEXT NOT NULL,
                total_rows_read INTEGER DEFAULT 0,
                rows_imported INTEGER DEFAULT 0,
                rows_skipped INTEGER DEFAULT 0,
                sparse_rows_count INTEGER DEFAULT 0,
                playtest_rows_count INTEGER DEFAULT 0,
                error_message TEXT
            );

            CREATE TABLE IF NOT EXISTS data_quality_issues (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ingestion_run_id INTEGER REFERENCES ingestion_runs(id) ON DELETE CASCADE,
                app_id INTEGER,
                game_name TEXT,
                issue_type TEXT NOT NULL,
                severity TEXT NOT NULL,
                details TEXT,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS games (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_id INTEGER NOT NULL UNIQUE,
                name TEXT NOT NULL,
                release_date TEXT,
                release_year INTEGER,
                estimated_owners_raw TEXT,
                owner_lower INTEGER DEFAULT 0,
                owner_upper INTEGER DEFAULT 0,
                owner_midpoint INTEGER DEFAULT 0,
                price REAL DEFAULT 0.0,
                peak_ccu INTEGER DEFAULT 0,
                positive INTEGER DEFAULT 0,
                negative INTEGER DEFAULT 0,
                recommendations INTEGER DEFAULT 0,
                average_playtime_forever INTEGER DEFAULT 0,
                median_playtime_forever INTEGER DEFAULT 0,
                header_image TEXT,
                metacritic_score INTEGER DEFAULT 0,
                is_sparse INTEGER NOT NULL DEFAULT 0,
                is_likely_playtest INTEGER NOT NULL DEFAULT 0,
                raw_metadata TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS developers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                canonical_name TEXT NOT NULL,
                normalized_name TEXT NOT NULL,
                game_count INTEGER NOT NULL DEFAULT 0,
                publisher_count INTEGER NOT NULL DEFAULT 0,
                genre_count INTEGER NOT NULL DEFAULT 0,
                collaborator_count INTEGER NOT NULL DEFAULT 0,
                earliest_release_year INTEGER,
                latest_release_year INTEGER,
                total_owner_lower INTEGER DEFAULT 0,
                total_owner_upper INTEGER DEFAULT 0,
                total_owner_midpoint INTEGER DEFAULT 0,
                is_merged INTEGER NOT NULL DEFAULT 0,
                merged_into_id INTEGER REFERENCES developers(id),
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS publishers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                canonical_name TEXT NOT NULL,
                normalized_name TEXT NOT NULL,
                game_count INTEGER NOT NULL DEFAULT 0,
                developer_count INTEGER NOT NULL DEFAULT 0,
                genre_count INTEGER NOT NULL DEFAULT 0,
                earliest_release_year INTEGER,
                latest_release_year INTEGER,
                total_owner_lower INTEGER DEFAULT 0,
                total_owner_upper INTEGER DEFAULT 0,
                total_owner_midpoint INTEGER DEFAULT 0,
                shannon_entropy REAL DEFAULT 0.0,
                is_merged INTEGER NOT NULL DEFAULT 0,
                merged_into_id INTEGER REFERENCES publishers(id),
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS genres (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                game_count INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS game_developers (
                game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
                developer_id INTEGER NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
                PRIMARY KEY (game_id, developer_id)
            );

            CREATE TABLE IF NOT EXISTS game_publishers (
                game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
                publisher_id INTEGER NOT NULL REFERENCES publishers(id) ON DELETE CASCADE,
                PRIMARY KEY (game_id, publisher_id)
            );

            CREATE TABLE IF NOT EXISTS game_genres (
                game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
                genre_id INTEGER NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
                PRIMARY KEY (game_id, genre_id)
            );

            CREATE TABLE IF NOT EXISTS entity_aliases (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                raw_name TEXT NOT NULL,
                normalized_name TEXT NOT NULL,
                canonical_entity_id INTEGER NOT NULL,
                matching_method TEXT NOT NULL,
                confidence REAL NOT NULL DEFAULT 1.0,
                is_confirmed INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                UNIQUE(entity_type, raw_name)
            );

            CREATE TABLE IF NOT EXISTS entity_merge_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                source_entity_id INTEGER NOT NULL,
                target_entity_id INTEGER NOT NULL,
                source_canonical_name TEXT NOT NULL,
                target_canonical_name TEXT NOT NULL,
                reason TEXT,
                created_at TEXT NOT NULL,
                is_undone INTEGER NOT NULL DEFAULT 0,
                undone_at TEXT
            );

            CREATE TABLE IF NOT EXISTS entity_resolution_candidates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                entity_a_id INTEGER NOT NULL,
                entity_b_id INTEGER NOT NULL,
                similarity_score REAL NOT NULL,
                matching_explanation TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                reviewed_at TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(entity_type, entity_a_id, entity_b_id)
            );

            -- Performance Indexes
            CREATE INDEX IF NOT EXISTS idx_games_app_id ON games(app_id);
            CREATE INDEX IF NOT EXISTS idx_games_release_year ON games(release_year);
            CREATE INDEX IF NOT EXISTS idx_games_name ON games(name);
            CREATE INDEX IF NOT EXISTS idx_games_is_sparse ON games(is_sparse);
            CREATE INDEX IF NOT EXISTS idx_games_is_playtest ON games(is_likely_playtest);

            CREATE INDEX IF NOT EXISTS idx_developers_canonical ON developers(canonical_name);
            CREATE INDEX IF NOT EXISTS idx_developers_normalized ON developers(normalized_name);
            CREATE INDEX IF NOT EXISTS idx_developers_game_count ON developers(game_count);
            CREATE INDEX IF NOT EXISTS idx_developers_is_merged ON developers(is_merged);

            CREATE INDEX IF NOT EXISTS idx_publishers_canonical ON publishers(canonical_name);
            CREATE INDEX IF NOT EXISTS idx_publishers_normalized ON publishers(normalized_name);
            CREATE INDEX IF NOT EXISTS idx_publishers_game_count ON publishers(game_count);
            CREATE INDEX IF NOT EXISTS idx_publishers_is_merged ON publishers(is_merged);

            CREATE INDEX IF NOT EXISTS idx_game_devs_dev ON game_developers(developer_id);
            CREATE INDEX IF NOT EXISTS idx_game_devs_game ON game_developers(game_id);
            CREATE INDEX IF NOT EXISTS idx_game_pubs_pub ON game_publishers(publisher_id);
            CREATE INDEX IF NOT EXISTS idx_game_pubs_game ON game_publishers(game_id);
            CREATE INDEX IF NOT EXISTS idx_game_genres_genre ON game_genres(genre_id);
            CREATE INDEX IF NOT EXISTS idx_game_genres_game ON game_genres(game_id);

            CREATE INDEX IF NOT EXISTS idx_aliases_raw ON entity_aliases(entity_type, raw_name);
            CREATE INDEX IF NOT EXISTS idx_aliases_canonical ON entity_aliases(entity_type, canonical_entity_id);
            CREATE INDEX IF NOT EXISTS idx_er_candidates_status ON entity_resolution_candidates(entity_type, status);
            CREATE INDEX IF NOT EXISTS idx_quality_issues_run ON data_quality_issues(ingestion_run_id);
            CREATE INDEX IF NOT EXISTS idx_quality_issues_type ON data_quality_issues(issue_type);

            INSERT INTO schema_version (version, applied_at) VALUES (1, datetime('now'));
            ",
        )?;
        tx.commit()?;
        info!("Applied migration v1 successfully.");
    }

    if current_version < 2 {
        let tx = conn.transaction()?;
        // Add transferred_game_ids to entity_merge_events so undo_merge can restore exact relationships
        let has_col: bool = tx
            .prepare("PRAGMA table_info(entity_merge_events)")?
            .query_map([], |r| r.get::<_, String>(1))?
            .filter_map(|r| r.ok())
            .any(|name| name == "transferred_game_ids");

        if !has_col {
            tx.execute_batch(
                "ALTER TABLE entity_merge_events ADD COLUMN transferred_game_ids TEXT;",
            )?;
        }
        tx.execute_batch(
            "INSERT OR REPLACE INTO schema_version (version, applied_at) VALUES (2, datetime('now'));",
        )?;
        tx.commit()?;
        info!("Applied migration v2 successfully.");
    }

    Ok(())
}
