use clap::{Parser, Subcommand};
use std::path::PathBuf;
use steam_developer_graph::config::Config;
use steam_developer_graph::db::init_db;
use steam_developer_graph::entity_resolution::EntityResolutionManager;
use steam_developer_graph::export::ExportService;
use steam_developer_graph::ingestion::{DatasetDownloader, DatasetIngester, QualityTracker};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "steam-graph")]
#[command(about = "Steam Developer Network Graph CLI tool for ingestion, resolution, and exports")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download dataset from Hugging Face
    Download {
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        dest: Option<PathBuf>,
    },
    /// Ingest a Parquet or JSON dataset file into the database
    Ingest {
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Show current database ecosystem and data quality status
    Status,
    /// Generate entity resolution candidates
    Resolve {
        #[arg(short, long, default_value = "developer")]
        entity_type: String,
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Export directory or graph data to CSV or JSON
    Export {
        #[arg(short, long)]
        target: String, // "developers", "publishers", "collaborations", "graph"
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    let config = Config::from_env();

    match cli.command {
        Commands::Download { url, dest } => {
            let target_path = dest.unwrap_or_else(|| config.dataset_path.clone());
            info!("Downloading Hugging Face dataset to {:?}", target_path);
            let res = DatasetDownloader::download_dataset(url.as_deref(), &target_path)
                .await
                .map_err(|e| -> Box<dyn std::error::Error> { e.to_string().into() })?;
            println!("Download complete!");
            println!("File: {:?}", res.file_path);
            println!("Size: {} bytes", res.file_size_bytes);
            println!("SHA-256: {}", res.sha256_checksum);
            println!("Duration: {:.2}s", res.duration_seconds);
        }
        Commands::Ingest { file } => {
            let target_file = file.unwrap_or_else(|| config.dataset_path.clone());
            if !target_file.exists() {
                eprintln!(
                    "Error: File {:?} does not exist. Please run 'download' or provide a valid path.",
                    target_file
                );
                std::process::exit(1);
            }

            println!("Ingesting dataset from {:?}", target_file);
            let pool = init_db(&config.database_path)?;
            let mut conn = pool.get()?;

            let stats = if target_file.extension().and_then(|e| e.to_str()) == Some("json")
                || target_file.extension().and_then(|e| e.to_str()) == Some("jsonl")
            {
                DatasetIngester::ingest_json_file(&mut conn, &target_file)?
            } else {
                DatasetIngester::ingest_parquet_file(&mut conn, &target_file)?
            };

            println!("\nIngestion Summary:");
            println!("==================");
            println!("Total Rows Read:      {}", stats.total_rows_read);
            println!("Rows Successfully Imported: {}", stats.rows_imported);
            println!("Rows Skipped:         {}", stats.rows_skipped);
            println!("Sparse Records:       {}", stats.sparse_rows);
            println!("Playtests Identified: {}", stats.playtest_rows);
            println!("Canonical Developers: {}", stats.total_developers);
            println!("Canonical Publishers: {}", stats.total_publishers);
            println!("Duration:             {:.2}s", stats.duration_seconds);
        }
        Commands::Status => {
            let pool = init_db(&config.database_path)?;
            let conn = pool.get()?;

            let total_games: i64 = conn
                .query_row("SELECT COUNT(*) FROM games", [], |r| r.get(0))
                .unwrap_or(0);
            let total_devs: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM developers WHERE is_merged = 0",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let total_pubs: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM publishers WHERE is_merged = 0",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let total_genres: i64 = conn
                .query_row("SELECT COUNT(*) FROM genres", [], |r| r.get(0))
                .unwrap_or(0);

            println!("\nSteam Developer Network Graph Status:");
            println!("=====================================");
            println!("Database:   {:?}", config.database_path);
            println!("Games:      {}", total_games);
            println!("Developers: {}", total_devs);
            println!("Publishers: {}", total_pubs);
            println!("Genres:     {}", total_genres);

            if total_games > 0 {
                let report = QualityTracker::generate_report(&conn)?;
                println!("\nData Quality Summary:");
                println!("Sparse records:           {}", report.sparse_records);
                println!("Likely playtests:         {}", report.likely_playtests);
                println!(
                    "Complete credits games:   {}",
                    report.games_with_complete_credits
                );
                println!("Missing release dates:    {}", report.missing_release_dates);
            }
        }
        Commands::Resolve { entity_type, limit } => {
            let pool = init_db(&config.database_path)?;
            let mut conn = pool.get()?;
            let count =
                EntityResolutionManager::generate_candidates(&mut conn, &entity_type, limit)?;
            println!(
                "Generated {} resolution candidates for {}",
                count, entity_type
            );
        }
        Commands::Export { target, output } => {
            let pool = init_db(&config.database_path)?;
            let conn = pool.get()?;

            let data = match target.to_lowercase().as_str() {
                "developers" | "devs" => {
                    ExportService::export_developers_csv(&conn, None, None, None)?
                }
                "publishers" | "pubs" => {
                    ExportService::export_publishers_csv(&conn, None, None, None)?
                }
                "collaborations" | "collabs" => {
                    ExportService::export_collaborations_csv(&conn, 2, None, None, None)?
                }
                other => {
                    eprintln!(
                        "Unknown export target: {}. Options: developers, publishers, collaborations",
                        other
                    );
                    std::process::exit(1);
                }
            };

            std::fs::write(&output, data)?;
            println!("Export written to {:?}", output);
        }
    }

    Ok(())
}
