use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse};
use crate::graph::metrics::GraphMetricsService;
use crate::graph::model::{DeveloperCollaboration, PublisherDeveloperRelation};
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CollabQueryParams {
    pub min_shared: Option<i64>,
    pub genre: Option<String>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
    pub limit: Option<usize>,
}

pub async fn get_top_collaborations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CollabQueryParams>,
) -> Result<Json<ApiResponse<Vec<DeveloperCollaboration>>>, ApiError> {
    let conn = state.pool.get()?;
    let min_shared = params.min_shared.unwrap_or(2);
    let limit = params.limit.unwrap_or(50).min(200);

    let collabs = GraphMetricsService::get_top_collaborations(
        &conn,
        min_shared,
        params.genre.as_deref(),
        params.min_year,
        params.max_year,
        limit,
    )?;

    Ok(Json(ApiResponse::ok(collabs)))
}

pub async fn get_developer_pair_details(
    State(state): State<Arc<AppState>>,
    Path((dev1_id, dev2_id)): Path<(i64, i64)>,
) -> Result<Json<ApiResponse<DeveloperCollaboration>>, ApiError> {
    let conn = state.pool.get()?;
    let details = GraphMetricsService::get_developer_pair_details(&conn, dev1_id, dev2_id)?
        .ok_or_else(|| {
            ApiError::not_found(format!(
                "No collaboration found between Developer {} and Developer {}",
                dev1_id, dev2_id
            ))
        })?;
    Ok(Json(ApiResponse::ok(details)))
}

pub async fn get_publisher_developer_relations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CollabQueryParams>,
) -> Result<Json<ApiResponse<Vec<PublisherDeveloperRelation>>>, ApiError> {
    let conn = state.pool.get()?;
    let min_shared = params.min_shared.unwrap_or(2);
    let limit = params.limit.unwrap_or(50).min(200);

    let relations =
        GraphMetricsService::get_top_publisher_developer_relations(&conn, min_shared, limit)?;

    Ok(Json(ApiResponse::ok(relations)))
}
