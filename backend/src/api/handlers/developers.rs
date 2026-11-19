use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse, PaginatedResponse, PaginationParams};
use crate::db::queries::{DbQueries, DeveloperListItem, DeveloperProfile};
use crate::graph::model::NodeType;
use crate::graph::queries::{GraphFilterParams, GraphQueryService};
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct DeveloperFilterParams {
    pub genre: Option<String>,
    pub min_games: Option<i64>,
}

pub async fn list_developers(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<DeveloperFilterParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<DeveloperListItem>>>, ApiError> {
    let conn = state.pool.get()?;
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(25).min(100);

    let (items, total) = DbQueries::list_developers(
        &conn,
        pagination.search.as_deref(),
        filters.genre.as_deref(),
        filters.min_games,
        pagination.sort_by.as_deref(),
        pagination.sort_desc.unwrap_or(true),
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

pub async fn get_developer_profile(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<DeveloperProfile>>, ApiError> {
    let conn = state.pool.get()?;
    let profile = DbQueries::get_developer_profile(&conn, id)?
        .ok_or_else(|| ApiError::not_found(format!("Developer ID {} not found", id)))?;
    Ok(Json(ApiResponse::ok(profile)))
}

pub async fn get_developer_network(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Query(filters): Query<GraphFilterParams>,
) -> Result<Json<ApiResponse<crate::graph::model::Subgraph>>, ApiError> {
    let conn = state.pool.get()?;
    let subgraph =
        GraphQueryService::get_neighborhood(&conn, NodeType::Developer, id, 1, &filters)?;
    Ok(Json(ApiResponse::ok(subgraph)))
}
