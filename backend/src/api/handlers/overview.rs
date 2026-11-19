use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse};
use crate::db::queries::{DbQueries, OverviewSummary, SearchResultItem};
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

pub async fn get_overview_summary(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<OverviewSummary>>, ApiError> {
    let conn = state.pool.get()?;
    let summary = DbQueries::get_overview_summary(&conn)?;
    Ok(Json(ApiResponse::ok(summary)))
}

pub async fn handle_global_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<SearchResultItem>>>, ApiError> {
    let conn = state.pool.get()?;
    let limit = params.limit.unwrap_or(15);
    let results = DbQueries::global_search(&conn, &params.q, limit)?;
    Ok(Json(ApiResponse::ok(results)))
}
