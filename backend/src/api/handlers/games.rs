use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse, PaginatedResponse, PaginationParams};
use crate::db::queries::{DbQueries, GameDetail, GameSummaryItem};
use crate::graph::model::NodeType;
use crate::graph::queries::{GraphFilterParams, GraphQueryService};
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GameFilterParams {
    pub genre: Option<String>,
    pub developer_id: Option<i64>,
    pub publisher_id: Option<i64>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
    pub min_owners: Option<i64>,
    pub max_owners: Option<i64>,
    pub exclude_sparse: Option<bool>,
    pub exclude_playtests: Option<bool>,
}

pub async fn list_games(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<GameFilterParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<GameSummaryItem>>>, ApiError> {
    let conn = state.pool.get()?;
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(25).min(100);

    let (items, total) = DbQueries::list_games(
        &conn,
        pagination.search.as_deref(),
        filters.genre.as_deref(),
        filters.developer_id,
        filters.publisher_id,
        filters.min_year,
        filters.max_year,
        filters.min_owners,
        filters.max_owners,
        filters.exclude_sparse.unwrap_or(false),
        filters.exclude_playtests.unwrap_or(false),
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

pub async fn get_game_details(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<GameDetail>>, ApiError> {
    let conn = state.pool.get()?;
    let game = DbQueries::get_game_details(&conn, id)?
        .ok_or_else(|| ApiError::not_found(format!("Game ID or AppID {} not found", id)))?;
    Ok(Json(ApiResponse::ok(game)))
}

pub async fn get_game_network(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Query(filters): Query<GraphFilterParams>,
) -> Result<Json<ApiResponse<crate::graph::model::Subgraph>>, ApiError> {
    let conn = state.pool.get()?;
    let subgraph = GraphQueryService::get_neighborhood(&conn, NodeType::Game, id, 1, &filters)?;
    Ok(Json(ApiResponse::ok(subgraph)))
}
