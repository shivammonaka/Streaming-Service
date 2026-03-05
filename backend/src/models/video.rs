use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// This is what a Video looks like in our database and code
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub id: Uuid,
    pub slug: String,        // the shareable link token e.g. "xK92mPqR"
    pub status: VideoStatus,
    pub original_path: Option<String>,  // where raw upload is saved
    pub hls_path: Option<String>,       // where ffmpeg output is saved
    pub created_at: DateTime<Utc>,
}

// The 4 states a video can be in
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum VideoStatus {
    Pending,     // just uploaded, waiting for ffmpeg
    Processing,  // ffmpeg is running
    Ready,       // ffmpeg done, can be streamed
    Failed,      // something went wrong
}