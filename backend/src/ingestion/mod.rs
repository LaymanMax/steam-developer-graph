pub mod downloader;
pub mod parquet_loader;
pub mod parser;
pub mod quality;

pub use downloader::DatasetDownloader;
pub use parquet_loader::{DatasetIngester, IngestionStats, RawGameRecord};
pub use parser::{assess_game_quality, parse_estimated_owners, parse_release_date};
pub use quality::{QualityReport, QualityTracker};
