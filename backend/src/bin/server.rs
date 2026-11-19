use std::sync::Arc;
use steam_developer_graph::api::{AppState, create_router};
use steam_developer_graph::config::Config;
use steam_developer_graph::db::init_db;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    info!("Starting Steam Developer Network Graph backend...");
    info!(
        "Configuration: host={}, port={}, db={:?}, dataset={:?}",
        config.host, config.port, config.database_path, config.dataset_path
    );

    let pool = init_db(&config.database_path)?;

    let app_state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    let router = create_router(app_state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("HTTP API listening on http://{}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
