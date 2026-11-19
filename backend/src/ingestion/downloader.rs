use reqwest::Client;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{error, info};

pub const DEFAULT_DATASET_URL: &str = "https://huggingface.co/datasets/FronkonGames/steam-games-dataset/resolve/main/data/train-00000-of-00001.parquet";

#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub file_path: PathBuf,
    pub source_url: String,
    pub file_size_bytes: u64,
    pub sha256_checksum: String,
    pub duration_seconds: f64,
}

pub struct DatasetDownloader;

impl DatasetDownloader {
    pub async fn download_dataset(
        url: Option<&str>,
        dest_path: &Path,
    ) -> Result<DownloadResult, Box<dyn std::error::Error + Send + Sync>> {
        let target_url = url.unwrap_or(DEFAULT_DATASET_URL);
        info!("Starting dataset acquisition from: {}", target_url);

        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let start_time = std::time::Instant::now();
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(600))
            .build()?;

        let mut response = client.get(target_url).send().await.map_err(|e| {
            error!("Failed to connect to Hugging Face dataset URL: {}", e);
            e
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let err_msg = format!("HTTP error from Hugging Face: {}", status);
            error!("{}", err_msg);
            return Err(err_msg.into());
        }

        let total_size = response.content_length().unwrap_or(0);
        info!(
            "Download target size: {} bytes (~{:.1} MB)",
            total_size,
            total_size as f64 / 1_048_576.0
        );

        let mut file = File::create(dest_path)?;
        let mut hasher = Sha256::new();
        let mut downloaded_bytes: u64 = 0;
        let mut last_reported_mb = 0;

        while let Some(chunk) = response.chunk().await? {
            downloaded_bytes += chunk.len() as u64;
            hasher.update(&chunk);
            file.write_all(&chunk)?;

            let current_mb = (downloaded_bytes / (1024 * 1024 * 10)) * 10;
            if current_mb > last_reported_mb {
                last_reported_mb = current_mb;
                info!(
                    "Downloaded {:.1} MB / {:.1} MB ({:.1}%)",
                    downloaded_bytes as f64 / 1_048_576.0,
                    total_size as f64 / 1_048_576.0,
                    if total_size > 0 {
                        (downloaded_bytes as f64 / total_size as f64) * 100.0
                    } else {
                        0.0
                    }
                );
            }
        }

        file.flush()?;

        let checksum = hex::encode(hasher.finalize());
        let duration = start_time.elapsed().as_secs_f64();

        info!(
            "Dataset downloaded successfully to {:?}. Size: {} bytes, SHA-256: {}, Duration: {:.2}s",
            dest_path, downloaded_bytes, checksum, duration
        );

        Ok(DownloadResult {
            file_path: dest_path.to_path_buf(),
            source_url: target_url.to_string(),
            file_size_bytes: downloaded_bytes,
            sha256_checksum: checksum,
            duration_seconds: duration,
        })
    }

    /// Verifies existing file checksum and size
    pub fn inspect_file(file_path: &Path) -> Result<(u64, String), Box<dyn std::error::Error>> {
        use std::io::Read;
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];
        let mut size: u64 = 0;
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            size += n as u64;
            hasher.update(&buffer[..n]);
        }
        let checksum = hex::encode(hasher.finalize());
        Ok((size, checksum))
    }
}
