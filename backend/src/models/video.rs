use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a video entity in the system.
/// Maps directly to the `videos` table in PostgreSQL.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Video {
    pub id: Uuid,

    /// Short random token used in shareable URLs e.g. "xK92mPqR"
    pub slug: String,

    /// Current processing state of the video
    pub status: VideoStatus,

    /// Local path or S3 key of the raw uploaded file
    pub original_path: Option<String>,

    /// Local path or S3 key of the HLS manifest (index.m3u8)
    /// Only set when status is Ready
    pub hls_path: Option<String>,

    /// File size in bytes, set on upload
    pub size_bytes: Option<i64>,

    /// MIME type of the uploaded file e.g. "video/mp4"
    pub mime_type: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// The lifecycle states a video goes through.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum VideoStatus {
    /// Upload received, waiting for transcoding to start
    Pending,

    /// ffmpeg is actively transcoding the video
    Processing,

    /// Transcoding complete, video is streamable
    Ready,

    /// Transcoding failed, video cannot be streamed
    Failed,
}

impl std::fmt::Display for VideoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VideoStatus::Pending    => write!(f, "Pending"),
            VideoStatus::Processing => write!(f, "Processing"),
            VideoStatus::Ready      => write!(f, "Ready"),
            VideoStatus::Failed     => write!(f, "Failed"),
        }
    }
}