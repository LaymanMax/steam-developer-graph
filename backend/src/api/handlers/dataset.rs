use crate::api::types::{ApiError, ApiResponse};
use crate::config::Config;
use crate::db::DbPool;
use crate::ingestion::downloader::{DEFAULT_DATASET_URL, DatasetDownloader};
use crate::ingestion::parquet_loader::{DatasetIngester, IngestionStats};
use crate::ingestion::quality::{QualityReport, QualityTracker};
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

#[derive(Debug, Serialize)]
pub struct DatasetStatusResponse {
    pub has_data: bool,
    pub total_games: i64,
    pub total_developers: i64,
    pub total_publishers: i64,
    pub total_genres: i64,
    pub local_parquet_exists: bool,
    pub local_parquet_path: String,
    pub local_parquet_size_bytes: Option<u64>,
    pub local_parquet_checksum: Option<String>,
    pub default_hf_url: String,
    pub latest_ingestion_run: Option<IngestionRunSummary>,
}

#[derive(Debug, Serialize)]
pub struct IngestionRunSummary {
    pub id: i64,
    pub run_type: String,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_rows_read: i64,
    pub rows_imported: i64,
    pub sparse_rows_count: i64,
    pub playtest_rows_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadRequest {
    pub source_url: Option<String>,
}

pub async fn get_dataset_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<DatasetStatusResponse>>, ApiError> {
    let conn = state.pool.get()?;

    let total_games: i64 = conn
        .query_row("SELECT COUNT(*) FROM games", [], |r| r.get(0))
        .unwrap_or(0);
    let total_developers: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM developers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let total_publishers: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM publishers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let total_genres: i64 = conn
        .query_row("SELECT COUNT(*) FROM genres", [], |r| r.get(0))
        .unwrap_or(0);

    let parquet_path = &state.config.dataset_path;
    let local_parquet_exists = parquet_path.exists();
    let (parquet_size, parquet_checksum) = if local_parquet_exists {
        DatasetDownloader::inspect_file(parquet_path)
            .map(|(s, c)| (Some(s), Some(c)))
            .unwrap_or((None, None))
    } else {
        (None, None)
    };

    let latest_run = conn
        .query_row(
            "SELECT id, run_type, status, started_at, completed_at, total_rows_read, rows_imported, sparse_rows_count, playtest_rows_count
             FROM ingestion_runs ORDER BY id DESC LIMIT 1",
            [],
            |r| {
                Ok(IngestionRunSummary {
                    id: r.get(0)?,
                    run_type: r.get(1)?,
                    status: r.get(2)?,
                    started_at: r.get(3)?,
                    completed_at: r.get(4)?,
                    total_rows_read: r.get(5)?,
                    rows_imported: r.get(6)?,
                    sparse_rows_count: r.get(7)?,
                    playtest_rows_count: r.get(8)?,
                })
            },
        )
        .ok();

    Ok(Json(ApiResponse::ok(DatasetStatusResponse {
        has_data: total_games > 0,
        total_games,
        total_developers,
        total_publishers,
        total_genres,
        local_parquet_exists,
        local_parquet_path: parquet_path.to_string_lossy().to_string(),
        local_parquet_size_bytes: parquet_size,
        local_parquet_checksum: parquet_checksum,
        default_hf_url: DEFAULT_DATASET_URL.to_string(),
        latest_ingestion_run: latest_run,
    })))
}

pub async fn get_data_quality_report(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<QualityReport>>, ApiError> {
    let conn = state.pool.get()?;
    let report = QualityTracker::generate_report(&conn)?;
    Ok(Json(ApiResponse::ok(report)))
}

pub async fn trigger_import(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ImportRequest>,
) -> Result<Json<ApiResponse<IngestionStats>>, ApiError> {
    let target_path = payload
        .file_path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| state.config.dataset_path.clone());

    if !target_path.exists() {
        return Err(ApiError::bad_request(format!(
            "Dataset file not found at: {:?}. Please acquire or provide a valid Parquet or JSON dataset file.",
            target_path
        )));
    }

    info!("Triggering dataset ingestion from: {:?}", target_path);

    let state_clone = Arc::clone(&state);
    let path_clone = target_path.clone();

    // Run ingestion in a blocking task since it performs heavy file I/O and DB transactions
    let stats = tokio::task::spawn_blocking(move || {
        let mut conn = state_clone
            .pool
            .get()
            .map_err(|e| format!("DB pool error: {}", e))?;

        if path_clone.extension().and_then(|ext| ext.to_str()) == Some("json")
            || path_clone.extension().and_then(|ext| ext.to_str()) == Some("jsonl")
        {
            DatasetIngester::ingest_json_file(&mut conn, &path_clone)
                .map_err(|e| format!("Ingestion failed: {}", e))
        } else {
            DatasetIngester::ingest_parquet_file(&mut conn, &path_clone)
                .map_err(|e| format!("Ingestion failed: {}", e))
        }
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task execution panicked: {}", e)))?
    .map_err(ApiError::internal)?;

    Ok(Json(ApiResponse::ok(stats)))
}

pub async fn trigger_download(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DownloadRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let dest_path = state.config.dataset_path.clone();
    let url = payload.source_url;

    info!("Triggering dataset download to {:?}", dest_path);

    tokio::task::spawn(async move {
        match DatasetDownloader::download_dataset(url.as_deref(), &dest_path).await {
            Ok(res) => {
                info!("Dataset download finished successfully: {:?}", res);
            }
            Err(e) => {
                tracing::error!("Dataset download failed: {}", e);
            }
        }
    });

    Ok(Json(ApiResponse::ok(
        "Dataset download initiated in background. Check dataset status for progress and completion.".to_string(),
    )))
}
