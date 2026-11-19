use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse};
use crate::graph::metrics::{GenreYearTrend, GraphMetricsService, HistoricalYearActivity};
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GenreTrendParams {
    pub top_n: Option<usize>,
}

pub async fn get_historical_timeline(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<HistoricalYearActivity>>>, ApiError> {
    let conn = state.pool.get()?;
    let timeline = GraphMetricsService::get_historical_timeline(&conn)?;
    Ok(Json(ApiResponse::ok(timeline)))
}

pub async fn get_genre_growth(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GenreTrendParams>,
) -> Result<Json<ApiResponse<Vec<GenreYearTrend>>>, ApiError> {
    let conn = state.pool.get()?;
    let top_n = params.top_n.unwrap_or(8).min(20);
    let trends = GraphMetricsService::get_genre_growth(&conn, top_n)?;
    Ok(Json(ApiResponse::ok(trends)))
}
