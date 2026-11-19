use crate::api::handlers::{
    AppState, collaboration, dataset, developers, entity_resolution, export, games, graph,
    historical, overview, publishers,
};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Dataset & Ingestion
        .route("/api/dataset/status", get(dataset::get_dataset_status))
        .route(
            "/api/dataset/quality",
            get(dataset::get_data_quality_report),
        )
        .route("/api/dataset/import", post(dataset::trigger_import))
        .route("/api/dataset/download", post(dataset::trigger_download))
        // Overview & Search
        .route("/api/overview/summary", get(overview::get_overview_summary))
        .route("/api/search", get(overview::handle_global_search))
        // Developers
        .route("/api/developers", get(developers::list_developers))
        .route(
            "/api/developers/{id}",
            get(developers::get_developer_profile),
        )
        .route(
            "/api/developers/{id}/network",
            get(developers::get_developer_network),
        )
        // Publishers
        .route("/api/publishers", get(publishers::list_publishers))
        .route(
            "/api/publishers/{id}",
            get(publishers::get_publisher_profile),
        )
        .route(
            "/api/publishers/{id}/network",
            get(publishers::get_publisher_network),
        )
        // Games
        .route("/api/games", get(games::list_games))
        .route("/api/games/{id}", get(games::get_game_details))
        .route("/api/games/{id}/network", get(games::get_game_network))
        // Graph
        .route("/api/graph/overview", get(graph::get_graph_overview))
        .route("/api/graph/neighborhood", get(graph::query_neighborhood))
        // Collaborations & Relationships
        .route(
            "/api/collaborations",
            get(collaboration::get_top_collaborations),
        )
        .route(
            "/api/collaborations/{dev1_id}/{dev2_id}",
            get(collaboration::get_developer_pair_details),
        )
        .route(
            "/api/relationships/publisher-developer",
            get(collaboration::get_publisher_developer_relations),
        )
        // Historical Trends
        .route(
            "/api/historical/timeline",
            get(historical::get_historical_timeline),
        )
        .route("/api/historical/genres", get(historical::get_genre_growth))
        // Entity Resolution
        .route(
            "/api/entity-resolution/candidates",
            get(entity_resolution::list_candidates),
        )
        .route(
            "/api/entity-resolution/report",
            get(entity_resolution::get_resolution_report),
        )
        .route(
            "/api/entity-resolution/history",
            get(entity_resolution::get_merge_history),
        )
        .route(
            "/api/entity-resolution/merge",
            post(entity_resolution::merge_entities),
        )
        .route(
            "/api/entity-resolution/reject",
            post(entity_resolution::reject_candidate),
        )
        .route(
            "/api/entity-resolution/mark-distinct",
            post(entity_resolution::mark_distinct),
        )
        .route(
            "/api/entity-resolution/assign-alias",
            post(entity_resolution::assign_alias),
        )
        .route(
            "/api/entity-resolution/undo",
            post(entity_resolution::undo_merge),
        )
        .route(
            "/api/entity-resolution/generate",
            post(entity_resolution::trigger_candidate_generation),
        )
        // Exports
        .route("/api/export/developers", get(export::export_developers))
        .route("/api/export/publishers", get(export::export_publishers))
        .route(
            "/api/export/collaborations",
            get(export::export_collaborations),
        )
        .route("/api/export/graph", get(export::export_graph_json))
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
