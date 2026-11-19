use crate::api::handlers::dataset::AppState;
use crate::api::types::ApiError;
use crate::export::ExportService;
use crate::graph::model::NodeType;
use crate::graph::queries::{GraphFilterParams, GraphQueryService};
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::response::IntoResponse;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct ExportDirParams {
    pub search: Option<String>,
    pub genre: Option<String>,
    pub min_games: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportCollabParams {
    pub min_shared: Option<i64>,
    pub genre: Option<String>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ExportGraphParams {
    pub node_type: Option<String>,
    pub id: Option<i64>,
}

pub async fn export_developers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportDirParams>,
) -> Result<impl IntoResponse, ApiError> {
    let conn = state.pool.get()?;
    let csv_data = ExportService::export_developers_csv(
        &conn,
        params.search.as_deref(),
        params.genre.as_deref(),
        params.min_games,
    )
    .map_err(|e| ApiError::internal(format!("Export failed: {}", e)))?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/csv; charset=utf-8".parse().unwrap());
    headers.insert(
        CONTENT_DISPOSITION,
        "attachment; filename=\"steam_developers.csv\""
            .parse()
            .unwrap(),
    );

    Ok((headers, csv_data))
}

pub async fn export_publishers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportDirParams>,
) -> Result<impl IntoResponse, ApiError> {
    let conn = state.pool.get()?;
    let csv_data = ExportService::export_publishers_csv(
        &conn,
        params.search.as_deref(),
        params.genre.as_deref(),
        params.min_games,
    )
    .map_err(|e| ApiError::internal(format!("Export failed: {}", e)))?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/csv; charset=utf-8".parse().unwrap());
    headers.insert(
        CONTENT_DISPOSITION,
        "attachment; filename=\"steam_publishers.csv\""
            .parse()
            .unwrap(),
    );

    Ok((headers, csv_data))
}

pub async fn export_collaborations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportCollabParams>,
) -> Result<impl IntoResponse, ApiError> {
    let conn = state.pool.get()?;
    let csv_data = ExportService::export_collaborations_csv(
        &conn,
        params.min_shared.unwrap_or(2),
        params.genre.as_deref(),
        params.min_year,
        params.max_year,
    )
    .map_err(|e| ApiError::internal(format!("Export failed: {}", e)))?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/csv; charset=utf-8".parse().unwrap());
    headers.insert(
        CONTENT_DISPOSITION,
        "attachment; filename=\"steam_developer_collaborations.csv\""
            .parse()
            .unwrap(),
    );

    Ok((headers, csv_data))
}

pub async fn export_graph_json(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportGraphParams>,
) -> Result<impl IntoResponse, ApiError> {
    let conn = state.pool.get()?;

    let subgraph = if let (Some(nt), Some(id)) = (params.node_type, params.id) {
        let n_type = match nt.to_lowercase().as_str() {
            "developer" | "dev" => NodeType::Developer,
            "publisher" | "pub" => NodeType::Publisher,
            _ => NodeType::Game,
        };
        GraphQueryService::get_neighborhood(&conn, n_type, id, 1, &GraphFilterParams::default())?
    } else {
        GraphQueryService::get_ecosystem_overview(&conn, &GraphFilterParams::default())?
    };

    let json_data = ExportService::export_graph_json(&subgraph)
        .map_err(|e| ApiError::internal(format!("Export failed: {}", e)))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        "application/json; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        CONTENT_DISPOSITION,
        "attachment; filename=\"steam_network_subgraph.json\""
            .parse()
            .unwrap(),
    );

    Ok((headers, json_data))
}
