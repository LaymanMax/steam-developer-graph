use crate::api::handlers::dataset::AppState;
use crate::api::types::{ApiError, ApiResponse};
use crate::graph::model::{NodeType, Subgraph};
use crate::graph::queries::{GraphFilterParams, GraphQueryService};
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct NeighborhoodQueryParams {
    pub node_type: String, // "developer", "publisher", "game"
    pub id: i64,
    pub depth: Option<usize>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
    pub genre: Option<String>,
    pub min_collaborations: Option<i64>,
    pub exclude_sparse: Option<bool>,
    pub exclude_playtests: Option<bool>,
    pub limit: Option<usize>,
}

pub async fn get_graph_overview(
    State(state): State<Arc<AppState>>,
    Query(filters): Query<GraphFilterParams>,
) -> Result<Json<ApiResponse<Subgraph>>, ApiError> {
    let conn = state.pool.get()?;
    let subgraph = GraphQueryService::get_ecosystem_overview(&conn, &filters)?;
    Ok(Json(ApiResponse::ok(subgraph)))
}

pub async fn query_neighborhood(
    State(state): State<Arc<AppState>>,
    Query(params): Query<NeighborhoodQueryParams>,
) -> Result<Json<ApiResponse<Subgraph>>, ApiError> {
    let conn = state.pool.get()?;
    let n_type = match params.node_type.to_lowercase().as_str() {
        "developer" | "dev" => NodeType::Developer,
        "publisher" | "pub" => NodeType::Publisher,
        "game" => NodeType::Game,
        other => {
            return Err(ApiError::bad_request(format!(
                "Invalid node_type '{}'. Expected 'developer', 'publisher', or 'game'",
                other
            )));
        }
    };

    let filters = GraphFilterParams {
        min_year: params.min_year,
        max_year: params.max_year,
        genre: params.genre,
        min_collaborations: params.min_collaborations,
        exclude_sparse: params.exclude_sparse.unwrap_or(false),
        exclude_playtests: params.exclude_playtests.unwrap_or(false),
        limit: params.limit,
    };

    let depth = params.depth.unwrap_or(1).min(2);
    let subgraph = GraphQueryService::get_neighborhood(&conn, n_type, params.id, depth, &filters)?;
    Ok(Json(ApiResponse::ok(subgraph)))
}
