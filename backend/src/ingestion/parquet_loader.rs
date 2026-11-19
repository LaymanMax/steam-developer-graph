use crate::entity_resolution::manager::EntityResolutionManager;
use crate::entity_resolution::normalizer::{
    normalize_conservative, normalize_key, parse_delimited_entities,
};
use crate::ingestion::downloader::DatasetDownloader;
use crate::ingestion::parser::{assess_game_quality, parse_estimated_owners, parse_release_date};
use crate::ingestion::quality::QualityTracker;
use arrow::array::{Array, AsArray};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionStats {
    pub run_id: i64,
    pub total_rows_read: usize,
    pub rows_imported: usize,
    pub rows_skipped: usize,
    pub sparse_rows: usize,
    pub playtest_rows: usize,
    pub total_developers: usize,
    pub total_publishers: usize,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawGameRecord {
    pub app_id: i64,
    pub name: String,
    pub release_date: String,
    pub estimated_owners: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub genres: Vec<String>,
    pub price: f64,
    pub peak_ccu: i64,
    pub positive: i64,
    pub negative: i64,
    pub recommendations: i64,
    pub average_playtime_forever: i64,
    pub median_playtime_forever: i64,
    pub header_image: Option<String>,
    pub metacritic_score: i64,
    pub raw_metadata: Option<String>,
}

pub struct DatasetIngester;

impl DatasetIngester {
    /// Ingests records from an Arrow/Parquet file directly into SQLite.
    pub fn ingest_parquet_file(
        conn: &mut Connection,
        parquet_path: &Path,
    ) -> Result<IngestionStats, Box<dyn std::error::Error>> {
        info!("Opening Parquet file for ingestion: {:?}", parquet_path);
        let start_time = std::time::Instant::now();

        let (file_size, checksum) =
            DatasetDownloader::inspect_file(parquet_path).unwrap_or((0, "unknown".to_string()));

        // Start ingestion run in DB
        let run_id = Self::create_ingestion_run(
            conn,
            "parquet_file",
            None,
            Some(&parquet_path.to_string_lossy()),
            file_size as i64,
            &checksum,
        )?;

        let file = File::open(parquet_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let mut reader = builder.build()?;

        let mut all_records = Vec::new();

        while let Some(batch_res) = reader.next() {
            let batch = batch_res?;
            let num_rows = batch.num_rows();

            // Locate columns dynamically (case-insensitive)
            let schema = batch.schema();
            let mut col_map: HashMap<String, usize> = HashMap::new();
            for (idx, field) in schema.fields().iter().enumerate() {
                col_map.insert(field.name().to_lowercase(), idx);
            }

            for row_idx in 0..num_rows {
                let app_id =
                    Self::get_int_or_str_column(&batch, &col_map, &["appid", "app_id"], row_idx)
                        .unwrap_or(0);
                let name = Self::get_string_column(&batch, &col_map, &["name", "title"], row_idx)
                    .unwrap_or_else(|| format!("App {}", app_id));
                let release_date = Self::get_string_column(
                    &batch,
                    &col_map,
                    &["release_date", "releasedate"],
                    row_idx,
                )
                .unwrap_or_default();
                let estimated_owners = Self::get_string_column(
                    &batch,
                    &col_map,
                    &["estimated_owners", "owners"],
                    row_idx,
                )
                .unwrap_or_default();

                let developers = Self::get_string_list_or_delimited(
                    &batch,
                    &col_map,
                    &["developers", "developer"],
                    row_idx,
                );
                let publishers = Self::get_string_list_or_delimited(
                    &batch,
                    &col_map,
                    &["publishers", "publisher"],
                    row_idx,
                );
                let genres = Self::get_string_list_or_delimited(
                    &batch,
                    &col_map,
                    &["genres", "genre"],
                    row_idx,
                );

                let price =
                    Self::get_float_column(&batch, &col_map, &["price"], row_idx).unwrap_or(0.0);
                let peak_ccu =
                    Self::get_int_column(&batch, &col_map, &["peak_ccu", "ccu"], row_idx)
                        .unwrap_or(0);
                let positive =
                    Self::get_int_column(&batch, &col_map, &["positive"], row_idx).unwrap_or(0);
                let negative =
                    Self::get_int_column(&batch, &col_map, &["negative"], row_idx).unwrap_or(0);
                let recommendations =
                    Self::get_int_column(&batch, &col_map, &["recommendations"], row_idx)
                        .unwrap_or(0);
                let average_playtime_forever =
                    Self::get_int_column(&batch, &col_map, &["average_playtime_forever"], row_idx)
                        .unwrap_or(0);
                let median_playtime_forever =
                    Self::get_int_column(&batch, &col_map, &["median_playtime_forever"], row_idx)
                        .unwrap_or(0);
                let header_image =
                    Self::get_string_column(&batch, &col_map, &["header_image"], row_idx);
                let metacritic_score =
                    Self::get_int_column(&batch, &col_map, &["metacritic_score"], row_idx)
                        .unwrap_or(0);

                all_records.push(RawGameRecord {
                    app_id,
                    name,
                    release_date,
                    estimated_owners,
                    developers,
                    publishers,
                    genres,
                    price,
                    peak_ccu,
                    positive,
                    negative,
                    recommendations,
                    average_playtime_forever,
                    median_playtime_forever,
                    header_image,
                    metacritic_score,
                    raw_metadata: None,
                });
            }
        }

        info!(
            "Parsed {} records from Parquet. Ingesting into database...",
            all_records.len()
        );
        let stats = Self::process_and_save_records(conn, run_id, all_records, start_time)?;
        Ok(stats)
    }

    /// Ingests records from a JSON array or JSONL file
    pub fn ingest_json_file(
        conn: &mut Connection,
        json_path: &Path,
    ) -> Result<IngestionStats, Box<dyn std::error::Error>> {
        info!("Opening JSON file for ingestion: {:?}", json_path);
        let start_time = std::time::Instant::now();

        let (file_size, checksum) =
            DatasetDownloader::inspect_file(json_path).unwrap_or((0, "unknown".to_string()));

        let run_id = Self::create_ingestion_run(
            conn,
            "json_file",
            None,
            Some(&json_path.to_string_lossy()),
            file_size as i64,
            &checksum,
        )?;

        let content = std::fs::read_to_string(json_path)?;
        let records: Vec<RawGameRecord> = if content.trim().starts_with('[') {
            serde_json::from_str(&content)?
        } else {
            // JSON Lines
            let mut list = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    if let Ok(rec) = serde_json::from_str::<RawGameRecord>(trimmed) {
                        list.push(rec);
                    }
                }
            }
            list
        };

        let stats = Self::process_and_save_records(conn, run_id, records, start_time)?;
        Ok(stats)
    }

    /// Process and save records in transactions
    pub fn process_and_save_records(
        conn: &mut Connection,
        run_id: i64,
        records: Vec<RawGameRecord>,
        start_time: std::time::Instant,
    ) -> Result<IngestionStats, Box<dyn std::error::Error>> {
        let total_rows_read = records.len();
        let mut rows_imported = 0;
        let mut rows_skipped = 0;
        let mut sparse_rows = 0;
        let mut playtest_rows = 0;

        // In-memory caches for fast entity and genre resolution
        let mut dev_alias_cache: HashMap<String, i64> = HashMap::new(); // normalized_name -> canonical_id
        let mut pub_alias_cache: HashMap<String, i64> = HashMap::new();
        let mut genre_cache: HashMap<String, i64> = HashMap::new();

        // Load existing aliases and genres
        {
            let mut stmt = conn.prepare(
                "SELECT normalized_name, canonical_entity_id FROM entity_aliases WHERE entity_type = 'developer'",
            )?;
            let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            for row in rows.flatten() {
                dev_alias_cache.insert(row.0, row.1);
            }

            let mut stmt_pub = conn.prepare(
                "SELECT normalized_name, canonical_entity_id FROM entity_aliases WHERE entity_type = 'publisher'",
            )?;
            let pub_rows = stmt_pub.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            for row in pub_rows.flatten() {
                pub_alias_cache.insert(row.0, row.1);
            }

            let mut stmt_g = conn.prepare("SELECT name, id FROM genres")?;
            let g_rows = stmt_g.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            for row in g_rows.flatten() {
                genre_cache.insert(row.0, row.1);
            }
        }

        // Process in batches
        let batch_size = 2000;
        for chunk in records.chunks(batch_size) {
            let tx = conn.transaction()?;

            for record in chunk {
                if record.app_id <= 0 {
                    rows_skipped += 1;
                    continue;
                }

                let parsed_date = parse_release_date(&record.release_date);
                let parsed_owners = parse_estimated_owners(&record.estimated_owners);

                let quality_flags = assess_game_quality(
                    record.app_id,
                    &record.name,
                    &parsed_date,
                    &parsed_owners,
                    &record.developers,
                    &record.publishers,
                    &record.genres,
                    record.price,
                    record.average_playtime_forever,
                );

                if quality_flags.is_sparse {
                    sparse_rows += 1;
                }
                if quality_flags.is_likely_playtest {
                    playtest_rows += 1;
                }

                // Insert or update game
                tx.execute(
                    "INSERT INTO games (
                        app_id, name, release_date, release_year, estimated_owners_raw,
                        owner_lower, owner_upper, owner_midpoint, price, peak_ccu,
                        positive, negative, recommendations, average_playtime_forever,
                        median_playtime_forever, header_image, metacritic_score,
                        is_sparse, is_likely_playtest, raw_metadata, created_at, updated_at
                     ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, datetime('now'), datetime('now')
                     ) ON CONFLICT(app_id) DO UPDATE SET
                        name = excluded.name,
                        release_date = excluded.release_date,
                        release_year = excluded.release_year,
                        estimated_owners_raw = excluded.estimated_owners_raw,
                        owner_lower = excluded.owner_lower,
                        owner_upper = excluded.owner_upper,
                        owner_midpoint = excluded.owner_midpoint,
                        price = excluded.price,
                        peak_ccu = excluded.peak_ccu,
                        positive = excluded.positive,
                        negative = excluded.negative,
                        recommendations = excluded.recommendations,
                        average_playtime_forever = excluded.average_playtime_forever,
                        median_playtime_forever = excluded.median_playtime_forever,
                        header_image = excluded.header_image,
                        metacritic_score = excluded.metacritic_score,
                        is_sparse = excluded.is_sparse,
                        is_likely_playtest = excluded.is_likely_playtest,
                        updated_at = datetime('now')
                     ",
                    params![
                        record.app_id,
                        record.name,
                        parsed_date.date_iso,
                        parsed_date.year,
                        parsed_owners.raw,
                        parsed_owners.lower as i64,
                        parsed_owners.upper as i64,
                        parsed_owners.midpoint as i64,
                        record.price,
                        record.peak_ccu,
                        record.positive,
                        record.negative,
                        record.recommendations,
                        record.average_playtime_forever,
                        record.median_playtime_forever,
                        record.header_image,
                        record.metacritic_score,
                        if quality_flags.is_sparse { 1 } else { 0 },
                        if quality_flags.is_likely_playtest { 1 } else { 0 },
                        record.raw_metadata,
                    ],
                )?;

                let game_id: i64 = tx.query_row(
                    "SELECT id FROM games WHERE app_id = ?1",
                    params![record.app_id],
                    |r| r.get(0),
                )?;

                // Process developers
                for raw_dev in &record.developers {
                    let conservative = normalize_conservative(raw_dev);
                    if conservative.is_empty() {
                        continue;
                    }
                    let norm = normalize_key(&conservative);

                    let dev_id = if let Some(&id) = dev_alias_cache.get(&norm) {
                        id
                    } else {
                        // Insert new developer
                        tx.execute(
                            "INSERT INTO developers (canonical_name, normalized_name, created_at, updated_at)
                             VALUES (?1, ?2, datetime('now'), datetime('now'))",
                            params![conservative, norm],
                        )?;
                        let new_id = tx.last_insert_rowid();

                        // Add alias
                        tx.execute(
                            "INSERT OR IGNORE INTO entity_aliases 
                             (entity_type, raw_name, normalized_name, canonical_entity_id, matching_method, confidence, is_confirmed, created_at)
                             VALUES ('developer', ?1, ?2, ?3, 'exact_normalized', 1.0, 1, datetime('now'))",
                            params![conservative, norm, new_id],
                        )?;

                        dev_alias_cache.insert(norm, new_id);
                        new_id
                    };

                    tx.execute(
                        "INSERT OR IGNORE INTO game_developers (game_id, developer_id) VALUES (?1, ?2)",
                        params![game_id, dev_id],
                    )?;
                }

                // Process publishers
                for raw_pub in &record.publishers {
                    let conservative = normalize_conservative(raw_pub);
                    if conservative.is_empty() {
                        continue;
                    }
                    let norm = normalize_key(&conservative);

                    let pub_id = if let Some(&id) = pub_alias_cache.get(&norm) {
                        id
                    } else {
                        tx.execute(
                            "INSERT INTO publishers (canonical_name, normalized_name, created_at, updated_at)
                             VALUES (?1, ?2, datetime('now'), datetime('now'))",
                            params![conservative, norm],
                        )?;
                        let new_id = tx.last_insert_rowid();

                        tx.execute(
                            "INSERT OR IGNORE INTO entity_aliases 
                             (entity_type, raw_name, normalized_name, canonical_entity_id, matching_method, confidence, is_confirmed, created_at)
                             VALUES ('publisher', ?1, ?2, ?3, 'exact_normalized', 1.0, 1, datetime('now'))",
                            params![conservative, norm, new_id],
                        )?;

                        pub_alias_cache.insert(norm, new_id);
                        new_id
                    };

                    tx.execute(
                        "INSERT OR IGNORE INTO game_publishers (game_id, publisher_id) VALUES (?1, ?2)",
                        params![game_id, pub_id],
                    )?;
                }

                // Process genres
                for raw_genre in &record.genres {
                    let cleaned = normalize_conservative(raw_genre);
                    if cleaned.is_empty() {
                        continue;
                    }

                    let genre_id = if let Some(&id) = genre_cache.get(&cleaned) {
                        id
                    } else {
                        tx.execute(
                            "INSERT OR IGNORE INTO genres (name) VALUES (?1)",
                            params![cleaned],
                        )?;
                        let gid: i64 = tx.query_row(
                            "SELECT id FROM genres WHERE name = ?1",
                            params![cleaned],
                            |r| r.get(0),
                        )?;
                        genre_cache.insert(cleaned.clone(), gid);
                        gid
                    };

                    tx.execute(
                        "INSERT OR IGNORE INTO game_genres (game_id, genre_id) VALUES (?1, ?2)",
                        params![game_id, genre_id],
                    )?;
                }

                // Record quality issues
                for (issue_type, severity, details) in quality_flags.issues {
                    let _ = QualityTracker::record_issue(
                        &tx,
                        Some(run_id),
                        record.app_id,
                        &record.name,
                        &issue_type,
                        &severity,
                        &details,
                    );
                }

                rows_imported += 1;
            }

            tx.commit()?;
            info!("Ingested {} / {} rows...", rows_imported, total_rows_read);
        }

        // Update genre game counts
        conn.execute(
            "UPDATE genres SET game_count = (
                SELECT COUNT(DISTINCT game_id) FROM game_genres WHERE genre_id = genres.id
            )",
            [],
        )?;

        // Recalculate metrics for all developers and publishers
        info!("Recalculating developer metrics...");
        conn.execute(
            "UPDATE developers SET
             game_count = (SELECT COUNT(DISTINCT game_id) FROM game_developers WHERE developer_id = developers.id),
             publisher_count = (
                 SELECT COUNT(DISTINCT gp.publisher_id)
                 FROM game_developers gd
                 JOIN game_publishers gp ON gd.game_id = gp.game_id
                 WHERE gd.developer_id = developers.id
             ),
             genre_count = (
                 SELECT COUNT(DISTINCT gg.genre_id)
                 FROM game_developers gd
                 JOIN game_genres gg ON gd.game_id = gg.game_id
                 WHERE gd.developer_id = developers.id
             ),
             collaborator_count = (
                 SELECT COUNT(DISTINCT gd2.developer_id)
                 FROM game_developers gd1
                 JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id != gd2.developer_id
                 WHERE gd1.developer_id = developers.id
             ),
             earliest_release_year = (
                 SELECT MIN(g.release_year)
                 FROM game_developers gd
                 JOIN games g ON gd.game_id = g.id
                 WHERE gd.developer_id = developers.id AND g.release_year IS NOT NULL AND g.release_year > 1970
             ),
             latest_release_year = (
                 SELECT MAX(g.release_year)
                 FROM game_developers gd
                 JOIN games g ON gd.game_id = g.id
                 WHERE gd.developer_id = developers.id AND g.release_year IS NOT NULL AND g.release_year > 1970
             ),
             total_owner_lower = (
                 SELECT COALESCE(SUM(g.owner_lower), 0)
                 FROM game_developers gd
                 JOIN games g ON gd.game_id = g.id
                 WHERE gd.developer_id = developers.id
             ),
             total_owner_upper = (
                 SELECT COALESCE(SUM(g.owner_upper), 0)
                 FROM game_developers gd
                 JOIN games g ON gd.game_id = g.id
                 WHERE gd.developer_id = developers.id
             ),
             total_owner_midpoint = (
                 SELECT COALESCE(SUM(g.owner_midpoint), 0)
                 FROM game_developers gd
                 JOIN games g ON gd.game_id = g.id
                 WHERE gd.developer_id = developers.id
             ),
             updated_at = datetime('now')",
            [],
        )?;

        info!("Recalculating publisher metrics and Shannon entropy...");
        conn.execute(
            "UPDATE publishers SET
             game_count = (SELECT COUNT(DISTINCT game_id) FROM game_publishers WHERE publisher_id = publishers.id),
             developer_count = (
                 SELECT COUNT(DISTINCT gd.developer_id)
                 FROM game_publishers gp
                 JOIN game_developers gd ON gp.game_id = gd.game_id
                 WHERE gp.publisher_id = publishers.id
             ),
             genre_count = (
                 SELECT COUNT(DISTINCT gg.genre_id)
                 FROM game_publishers gp
                 JOIN game_genres gg ON gp.game_id = gg.game_id
                 WHERE gp.publisher_id = publishers.id
             ),
             earliest_release_year = (
                 SELECT MIN(g.release_year)
                 FROM game_publishers gp
                 JOIN games g ON gp.game_id = g.id
                 WHERE gp.publisher_id = publishers.id AND g.release_year IS NOT NULL AND g.release_year > 1970
             ),
             latest_release_year = (
                 SELECT MAX(g.release_year)
                 FROM game_publishers gp
                 JOIN games g ON gp.game_id = g.id
                 WHERE gp.publisher_id = publishers.id AND g.release_year IS NOT NULL AND g.release_year > 1970
             ),
             total_owner_lower = (
                 SELECT COALESCE(SUM(g.owner_lower), 0)
                 FROM game_publishers gp
                 JOIN games g ON gp.game_id = g.id
                 WHERE gp.publisher_id = publishers.id
             ),
             total_owner_upper = (
                 SELECT COALESCE(SUM(g.owner_upper), 0)
                 FROM game_publishers gp
                 JOIN games g ON gp.game_id = g.id
                 WHERE gp.publisher_id = publishers.id
             ),
             total_owner_midpoint = (
                 SELECT COALESCE(SUM(g.owner_midpoint), 0)
                 FROM game_publishers gp
                 JOIN games g ON gp.game_id = g.id
                 WHERE gp.publisher_id = publishers.id
             ),
             updated_at = datetime('now')",
            [],
        )?;

        // Compute Shannon entropy for publishers with games
        let pub_ids: Vec<i64> = {
            let mut stmt = conn.prepare("SELECT id FROM publishers WHERE game_count > 0")?;
            stmt.query_map([], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect()
        };
        for pid in pub_ids {
            let _ = EntityResolutionManager::recalculate_entity_metrics(conn, "publisher", pid);
        }

        // Generate initial resolution candidates
        info!("Generating entity resolution candidate pairs...");
        let _ = EntityResolutionManager::generate_candidates(conn, "developer", 100);
        let _ = EntityResolutionManager::generate_candidates(conn, "publisher", 100);

        let duration = start_time.elapsed().as_secs_f64();

        // Complete ingestion run
        conn.execute(
            "UPDATE ingestion_runs SET
             completed_at = datetime('now'),
             status = 'completed',
             total_rows_read = ?1,
             rows_imported = ?2,
             rows_skipped = ?3,
             sparse_rows_count = ?4,
             playtest_rows_count = ?5
             WHERE id = ?6",
            params![
                total_rows_read as i64,
                rows_imported as i64,
                rows_skipped as i64,
                sparse_rows as i64,
                playtest_rows as i64,
                run_id
            ],
        )?;

        let total_developers: usize = conn.query_row(
            "SELECT COUNT(*) FROM developers WHERE is_merged = 0",
            [],
            |r| Ok(r.get::<_, i64>(0)? as usize),
        )?;
        let total_publishers: usize = conn.query_row(
            "SELECT COUNT(*) FROM publishers WHERE is_merged = 0",
            [],
            |r| Ok(r.get::<_, i64>(0)? as usize),
        )?;

        info!(
            "Ingestion completed in {:.2}s. Total imported: {}, Sparse: {}, Playtests: {}, Devs: {}, Pubs: {}",
            duration, rows_imported, sparse_rows, playtest_rows, total_developers, total_publishers
        );

        Ok(IngestionStats {
            run_id,
            total_rows_read,
            rows_imported,
            rows_skipped,
            sparse_rows,
            playtest_rows,
            total_developers,
            total_publishers,
            duration_seconds: duration,
        })
    }

    fn create_ingestion_run(
        conn: &Connection,
        run_type: &str,
        source_url: Option<&str>,
        file_path: Option<&str>,
        file_size: i64,
        checksum: &str,
    ) -> Result<i64> {
        conn.execute(
            "INSERT INTO ingestion_runs 
             (run_type, source_url, file_path, file_size_bytes, file_checksum, started_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), 'running')",
            params![run_type, source_url, file_path, file_size, checksum],
        )?;
        Ok(conn.last_insert_rowid())
    }

    // Helper functions for dynamic arrow record batch reading
    fn get_string_column(
        batch: &arrow::record_batch::RecordBatch,
        col_map: &HashMap<String, usize>,
        candidates: &[&str],
        row: usize,
    ) -> Option<String> {
        for cand in candidates {
            if let Some(&idx) = col_map.get(*cand) {
                let col = batch.column(idx);
                if col.is_null(row) {
                    return None;
                }
                if let Some(str_arr) = col.as_string_opt::<i32>() {
                    return Some(str_arr.value(row).to_string());
                }
                if let Some(str_arr) = col.as_string_opt::<i64>() {
                    return Some(str_arr.value(row).to_string());
                }
            }
        }
        None
    }

    fn get_int_column(
        batch: &arrow::record_batch::RecordBatch,
        col_map: &HashMap<String, usize>,
        candidates: &[&str],
        row: usize,
    ) -> Option<i64> {
        for cand in candidates {
            if let Some(&idx) = col_map.get(*cand) {
                let col = batch.column(idx);
                if col.is_null(row) {
                    return None;
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::Int64Type>() {
                    return Some(arr.value(row));
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::Int32Type>() {
                    return Some(arr.value(row) as i64);
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::UInt64Type>() {
                    return Some(arr.value(row) as i64);
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::UInt32Type>() {
                    return Some(arr.value(row) as i64);
                }
            }
        }
        None
    }

    fn get_int_or_str_column(
        batch: &arrow::record_batch::RecordBatch,
        col_map: &HashMap<String, usize>,
        candidates: &[&str],
        row: usize,
    ) -> Option<i64> {
        if let Some(val) = Self::get_int_column(batch, col_map, candidates, row) {
            return Some(val);
        }
        if let Some(str_val) = Self::get_string_column(batch, col_map, candidates, row) {
            return str_val.trim().parse::<i64>().ok();
        }
        None
    }

    fn get_float_column(
        batch: &arrow::record_batch::RecordBatch,
        col_map: &HashMap<String, usize>,
        candidates: &[&str],
        row: usize,
    ) -> Option<f64> {
        for cand in candidates {
            if let Some(&idx) = col_map.get(*cand) {
                let col = batch.column(idx);
                if col.is_null(row) {
                    return None;
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::Float64Type>() {
                    return Some(arr.value(row));
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::Float32Type>() {
                    return Some(arr.value(row) as f64);
                }
                if let Some(arr) = col.as_primitive_opt::<arrow::datatypes::Int64Type>() {
                    return Some(arr.value(row) as f64);
                }
            }
        }
        None
    }

    fn get_string_list_or_delimited(
        batch: &arrow::record_batch::RecordBatch,
        col_map: &HashMap<String, usize>,
        candidates: &[&str],
        row: usize,
    ) -> Vec<String> {
        for cand in candidates {
            if let Some(&idx) = col_map.get(*cand) {
                let col = batch.column(idx);
                if col.is_null(row) {
                    return Vec::new();
                }

                // If column is a ListArray
                if let Some(list_arr) = col.as_list_opt::<i32>() {
                    let sub_array = list_arr.value(row);
                    if let Some(str_arr) = sub_array.as_string_opt::<i32>() {
                        let mut res = Vec::new();
                        for i in 0..str_arr.len() {
                            if !str_arr.is_null(i) {
                                res.push(str_arr.value(i).to_string());
                            }
                        }
                        return res;
                    }
                }

                // Or if it's a string, parse delimited
                if let Some(s) = Self::get_string_column(batch, col_map, &[cand], row) {
                    return parse_delimited_entities(&s);
                }
            }
        }
        Vec::new()
    }
}
