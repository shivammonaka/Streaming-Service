use anyhow::Result;
use sqlx::PgPool;
use tokio::process::Command;
use tokio::fs;
use uuid::Uuid;
use crate::db;
use crate::models::video::VideoStatus;

/// Runs the full transcoding pipeline for a video.
/// Updates DB status throughout the process.
/// This runs as a background tokio task — errors are logged, not propagated.
pub async fn run(
    video_id: Uuid,
    input_path: &str,
    output_dir: &str,
    pool: &PgPool,
) {
    // mark as processing
    if let Err(e) = db::videos::update_status(pool, video_id, VideoStatus::Processing, None).await {
        tracing::error!("Failed to set Processing status for {}: {}", video_id, e);
        return;
    }

    tracing::info!("Starting transcoding for video: {}", video_id);

    match transcode(input_path, output_dir).await {
        Ok(_) => {
            let hls_path = format!("{}/index.m3u8", output_dir);
            tracing::info!("Transcoding complete: {}", hls_path);

            // delete raw upload to save disk space
            if let Err(e) = fs::remove_file(input_path).await {
                tracing::warn!("Failed to delete raw upload {}: {}", input_path, e);
            }

            if let Err(e) = db::videos::update_status(
                pool,
                video_id,
                VideoStatus::Ready,
                Some(hls_path),
            ).await {
                tracing::error!("Failed to set Ready status for {}: {}", video_id, e);
            }
        }
        Err(e) => {
            tracing::error!("Transcoding failed for {}: {}", video_id, e);

            if let Err(e) = db::videos::update_status(
                pool,
                video_id,
                VideoStatus::Failed,
                None,
            ).await {
                tracing::error!("Failed to set Failed status for {}: {}", video_id, e);
            }
        }
    }
}

/// Invokes ffmpeg to convert input file to HLS format.
/// Uses stream copy (no re-encoding) for maximum speed.
async fn transcode(input_path: &str, output_dir: &str) -> Result<()> {
    fs::create_dir_all(output_dir).await?;

    let output_path = format!("{}/index.m3u8", output_dir);

    let status = Command::new("ffmpeg")
        .arg("-loglevel").arg("error")
        .args([
            "-i", input_path,
            "-codec:", "copy",      // copy streams without re-encoding (fast)
            "-start_number", "0",
            "-hls_time", "10",      // 10 second chunks
            "-hls_list_size", "0",  // include all chunks in manifest
            "-f", "hls",
            &output_path,
        ])
        .status()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("ffmpeg exited with status: {}", status))
    }
}