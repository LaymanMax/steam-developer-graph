use std::path::PathBuf;
use steam_developer_graph::db::init_db;
use steam_developer_graph::db::queries::DbQueries;
use steam_developer_graph::entity_resolution::manager::EntityResolutionManager;
use steam_developer_graph::export::ExportService;
use steam_developer_graph::graph::metrics::GraphMetricsService;
use steam_developer_graph::graph::model::NodeType;
use steam_developer_graph::graph::queries::{GraphFilterParams, GraphQueryService};
use steam_developer_graph::ingestion::parquet_loader::DatasetIngester;
use steam_developer_graph::ingestion::quality::QualityTracker;

#[test]
fn test_full_pipeline_with_sample_fixture() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample_games.json");

    assert!(fixture_path.exists(), "Fixture file must exist");

    // Create a temporary SQLite database for this integration test
    let temp_dir = std::env::temp_dir().join(format!("steam_graph_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let db_path = temp_dir.join("test.db");
    let _ = std::fs::remove_file(&db_path);

    let pool = init_db(&db_path).expect("Failed to initialize test db");
    let mut conn = pool.get().expect("Failed to get db connection");

    // 1. Ingest fixture JSON
    let stats = DatasetIngester::ingest_json_file(&mut conn, &fixture_path)
        .expect("Ingestion must succeed");

    assert_eq!(stats.total_rows_read, 10);
    assert_eq!(stats.rows_imported, 10);
    assert_eq!(stats.rows_skipped, 0);
    assert!(stats.sparse_rows >= 1, "Must detect sparse record");
    assert!(stats.playtest_rows >= 2, "Must detect playtest records");

    // 2. Verify Database queries & overview
    let summary = DbQueries::get_overview_summary(&conn).expect("Overview summary must succeed");
    assert_eq!(summary.total_games, 10);
    assert!(summary.total_developers >= 4);
    assert!(summary.total_publishers >= 4);
    assert!(summary.total_genres >= 4);
    assert!(summary.dev_collab_relationships >= 1);

    // 3. Verify Data Quality Report
    let quality = QualityTracker::generate_report(&conn).expect("Quality report must succeed");
    assert_eq!(quality.total_games, 10);
    assert!(quality.sparse_records >= 1);
    assert!(quality.likely_playtests >= 2);
    assert!(quality.missing_developers >= 1); // Black Dragon Mage Playtest has empty devs

    // 4. Verify Developer Collaborations:
    // "Fallout: New Vegas" credits both "Obsidian Entertainment" and "Bethesda Game Studios"
    let collabs = GraphMetricsService::get_top_collaborations(&conn, 1, None, None, None, 10)
        .expect("Collaboration query must succeed");
    assert!(!collabs.is_empty(), "Must find at least 1 collaboration");
    let fallout_collab = collabs.iter().find(|c| {
        (c.dev1_name.contains("Obsidian") && c.dev2_name.contains("Bethesda"))
            || (c.dev1_name.contains("Bethesda") && c.dev2_name.contains("Obsidian"))
    });
    assert!(
        fallout_collab.is_some(),
        "Must identify Obsidian + Bethesda collaboration on Fallout: New Vegas"
    );
    let c = fallout_collab.unwrap();
    assert_eq!(c.shared_games_count, 1);

    // Verify Pair details query
    let pair_details = GraphMetricsService::get_developer_pair_details(&conn, c.dev1_id, c.dev2_id)
        .expect("Pair details must succeed")
        .expect("Pair details must be found");
    assert_eq!(pair_details.shared_games.len(), 1);
    assert_eq!(pair_details.shared_games[0].name, "Fallout: New Vegas");

    // 5. Verify Publisher Shannon Entropy Diversity:
    // Valve published Dota 2 (Action, Strategy), Portal (Action), etc.
    let (publishers, _) = DbQueries::list_publishers(&conn, None, None, None, None, true, 1, 10)
        .expect("List publishers must succeed");
    let valve_pub = publishers
        .iter()
        .find(|p| p.canonical_name.eq_ignore_ascii_case("valve"));
    assert!(valve_pub.is_some());
    let v = valve_pub.unwrap();
    assert!(v.shannon_entropy >= 0.0);

    // 6. Verify Candidate Detection (Entity Resolution):
    // Bethesda Softworks vs Bethesda Softworks LLC or Valve vs Valve Corporation
    let (candidates, total_candidates) = DbQueries::list_resolution_candidates(
        &conn,
        Some("publisher"),
        Some("pending"),
        None,
        1,
        10,
    )
    .expect("List candidates must succeed");
    assert!(
        total_candidates >= 1,
        "Must have generated at least 1 candidate match during ingestion"
    );
    assert!(!candidates.is_empty());
    let cand = &candidates[0];
    assert!(cand.similarity_score >= 0.85);

    // 7. Verify Entity Merge & Undo:
    let source_id = cand.entity_a_id;
    let target_id = cand.entity_b_id;
    let orig_source_games = cand.entity_a_games;
    let orig_target_games = cand.entity_b_games;

    EntityResolutionManager::merge_entities(
        &mut conn,
        "publisher",
        source_id,
        target_id,
        Some("Test merge"),
    )
    .expect("Merge must succeed");

    // Verify source is merged
    let is_merged: i64 = conn
        .query_row(
            "SELECT is_merged FROM publishers WHERE id = ?1",
            rusqlite::params![source_id],
            |r| r.get(0),
        )
        .expect("Query source merged status");
    assert_eq!(is_merged, 1);

    // Check merge history and undo
    let history = DbQueries::get_merge_history(&conn, 10).expect("Get merge history");
    assert!(!history.is_empty());
    let merge_event_id = history[0].id;

    EntityResolutionManager::undo_merge(&mut conn, merge_event_id)
        .expect("Undo merge must succeed");

    // Source must be unmerged and exact game counts must be restored
    let (is_unmerged, restored_source_games): (i64, i64) = conn
        .query_row(
            "SELECT is_merged, game_count FROM publishers WHERE id = ?1",
            rusqlite::params![source_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .expect("Query source unmerged status");
    assert_eq!(is_unmerged, 0);
    assert_eq!(
        restored_source_games, orig_source_games,
        "Undo merge must restore exact original game count to source entity"
    );

    let restored_target_games: i64 = conn
        .query_row(
            "SELECT game_count FROM publishers WHERE id = ?1",
            rusqlite::params![target_id],
            |r| r.get(0),
        )
        .expect("Query target game count");
    assert_eq!(
        restored_target_games, orig_target_games,
        "Undo merge must restore exact original game count to target entity"
    );

    // 8. Verify Graph Subgraph Query
    let graph = GraphQueryService::get_neighborhood(
        &conn,
        NodeType::Game,
        1, // Dota 2
        1,
        &GraphFilterParams::default(),
    )
    .expect("Graph query must succeed");
    assert!(!graph.nodes.is_empty());
    assert!(!graph.links.is_empty());

    // 9. Verify CSV and JSON Exports
    let dev_csv = ExportService::export_developers_csv(&conn, None, None, None)
        .expect("Export developers CSV must succeed");
    assert!(dev_csv.contains("Canonical Name"));
    assert!(dev_csv.contains("Valve"));

    let collab_csv = ExportService::export_collaborations_csv(&conn, 1, None, None, None)
        .expect("Export collabs CSV must succeed");
    assert!(collab_csv.contains("Shared Games Count"));

    let graph_json =
        ExportService::export_graph_json(&graph).expect("Export graph JSON must succeed");
    assert!(graph_json.contains("\"nodes\""));

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);
}
