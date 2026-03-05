use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use crate::AppState;
use crate::db;

#[derive(Serialize)]
pub struct VideoResponse {
    pub status: String,
    pub manifest_url: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// GET /v/:slug
/// Returns video status and manifest URL for the player.
/// Called when a user opens a share link.
pub async fn get_video(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {

    match db::videos::get_by_slug(&state.db, &slug).await {
        Ok(video) => Json(VideoResponse {
            status: video.status.to_string(),
            manifest_url: video.hls_path
                .map(|p| state.storage.public_url(&p)),
        }).into_response(),

        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: format!("Video not found: {}", slug) }),
        ).into_response(),
    }
}

/// GET /videos/:slug/*file
/// Serves HLS chunks (.ts) and manifest (.m3u8) files to the browser.
/// Called automatically by hls.js as it plays the video.
pub async fn serve_file(
    State(state): State<AppState>,
    Path((slug, file)): Path<(String, String)>,
) -> impl IntoResponse {

    // security: prevent path traversal attacks
    // e.g. reject "../../etc/passwd"
    if file.contains("..") || file.starts_with('/') {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid file path".to_string() }),
        ).into_response();
    }

    // build file path using storage backend
    let base_dir = state.storage.hls_output_path(&slug);
    let file_path = format!("{}/{}", base_dir, file);

    match tokio::fs::read(&file_path).await {
        Ok(contents) => {
            let content_type = if file.ends_with(".m3u8") {
                "application/vnd.apple.mpegurl"
            } else if file.ends_with(".ts") {
                "video/mp2t"
            } else {
                "application/octet-stream"
            };

            (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, content_type)],
                contents,
            ).into_response()
        }

        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: format!("File not found: {}", file) }),
        ).into_response(),
    }
}