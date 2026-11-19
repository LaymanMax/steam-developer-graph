use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::path::PathBuf;
use std::sync::Arc;
use steam_developer_graph::api::create_router;
use steam_developer_graph::config::Config;
use steam_developer_graph::db::init_db;
use steam_developer_graph::ingestion::parquet_loader::DatasetIngester;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_routes() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample_games.json");

    let temp_dir = std::env::temp_dir().join(format!("steam_api_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let db_path = temp_dir.join("api_test.db");
    let _ = std::fs::remove_file(&db_path);

    let pool = init_db(&db_path).expect("Failed to initialize test db");
    {
        let mut conn = pool.get().expect("Failed to get connection");
        DatasetIngester::ingest_json_file(&mut conn, &fixture_path)
            .expect("Ingestion must succeed");
    }

    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 3001,
        database_path: db_path.clone(),
        dataset_path: temp_dir.join("test.parquet"),
    };

    let state = Arc::new(steam_developer_graph::api::handlers::AppState { pool, config });
    let app = create_router(state);

    // 1. GET /api/dataset/status
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/dataset/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 2. GET /api/overview/summary
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/overview/summary")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 3. GET /api/developers
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/developers?page=1&page_size=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 4. GET /api/publishers
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/publishers?page=1&page_size=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 5. GET /api/games
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/games?page=1&page_size=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 6. GET /api/search?q=valve
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/search?q=valve")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 7. GET /api/collaborations
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/collaborations?min_shared=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 8. GET /api/graph/overview
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/graph/overview?limit=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);
}
