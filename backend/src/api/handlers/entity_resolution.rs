use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse, PaginatedResponse};
use crate::db::queries::{DbQueries, MergeHistoryItem, ResolutionCandidateItem, ResolutionReport};
use crate::entity_resolution::manager::EntityResolutionManager;
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CandidateQueryParams {
    pub entity_type: Option<String>,
    pub status: Option<String>,
    pub min_similarity: Option<f64>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
    pub entity_type: String,
    pub source_id: i64,
    pub target_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RejectRequest {
    pub entity_type: String,
    pub entity_a_id: i64,
    pub entity_b_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AssignAliasRequest {
    pub entity_type: String,
    pub raw_name: String,
    pub canonical_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UndoMergeRequest {
    pub merge_event_id: i64,
}

pub async fn list_candidates(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CandidateQueryParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<ResolutionCandidateItem>>>, ApiError> {
    let conn = state.pool.get()?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20).min(100);

    let (items, total) = DbQueries::list_resolution_candidates(
        &conn,
        params.entity_type.as_deref(),
        params.status.as_deref(),
        params.min_similarity,
        page,
        page_size,
    )?;

    let total_pages = ((total as f64) / (page_size as f64)).ceil() as usize;

    Ok(Json(ApiResponse::ok(PaginatedResponse {
        items,
        total,
        page,
        page_size,
        total_pages,
    })))
}

pub async fn get_resolution_report(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<ResolutionReport>>, ApiError> {
    let conn = state.pool.get()?;
    let report = DbQueries::get_resolution_report(&conn)?;
    Ok(Json(ApiResponse::ok(report)))
}

pub async fn get_merge_history(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<MergeHistoryItem>>>, ApiError> {
    let conn = state.pool.get()?;
    let history = DbQueries::get_merge_history(&conn, 100)?;
    Ok(Json(ApiResponse::ok(history)))
}

pub async fn merge_entities(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MergeRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let mut conn = state.pool.get()?;
    EntityResolutionManager::merge_entities(
        &mut conn,
        &payload.entity_type,
        payload.source_id,
        payload.target_id,
        payload.reason.as_deref(),
    )
    .map_err(|e| ApiError::bad_request(format!("Failed to merge entities: {}", e)))?;

    Ok(Json(ApiResponse::ok(format!(
        "Successfully merged {} ID {} into ID {}",
        payload.entity_type, payload.source_id, payload.target_id
    ))))
}

pub async fn reject_candidate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RejectRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let conn = state.pool.get()?;
    EntityResolutionManager::reject_candidate(
        &conn,
        &payload.entity_type,
        payload.entity_a_id,
        payload.entity_b_id,
    )?;

    Ok(Json(ApiResponse::ok("Candidate rejected".to_string())))
}

pub async fn mark_distinct(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RejectRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let conn = state.pool.get()?;
    EntityResolutionManager::mark_distinct(
        &conn,
        &payload.entity_type,
        payload.entity_a_id,
        payload.entity_b_id,
    )?;

    Ok(Json(ApiResponse::ok(
        "Entities marked as distinct".to_string(),
    )))
}

pub async fn assign_alias(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AssignAliasRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let conn = state.pool.get()?;
    EntityResolutionManager::assign_alias(
        &conn,
        &payload.entity_type,
        &payload.raw_name,
        payload.canonical_id,
    )?;

    Ok(Json(ApiResponse::ok(format!(
        "Alias '{}' assigned to {} ID {}",
        payload.raw_name, payload.entity_type, payload.canonical_id
    ))))
}

pub async fn undo_merge(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UndoMergeRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let mut conn = state.pool.get()?;
    EntityResolutionManager::undo_merge(&mut conn, payload.merge_event_id)
        .map_err(|e| ApiError::bad_request(format!("Failed to undo merge: {}", e)))?;

    Ok(Json(ApiResponse::ok(format!(
        "Merge event ID {} undone successfully",
        payload.merge_event_id
    ))))
}

pub async fn trigger_candidate_generation(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let mut conn = state.pool.get()?;
    let devs =
        EntityResolutionManager::generate_candidates(&mut conn, "developer", 150).unwrap_or(0);
    let pubs =
        EntityResolutionManager::generate_candidates(&mut conn, "publisher", 150).unwrap_or(0);

    Ok(Json(ApiResponse::ok(format!(
        "Generated {} developer and {} publisher candidates",
        devs, pubs
    ))))
}
