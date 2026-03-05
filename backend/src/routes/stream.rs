use axum::{
    extract::{Path, State},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Serialize;
use crate::AppState;
use crate::db;

#[derive(Serialize)]
pub struct VideoResponse {
    pub status: String,
    pub manifest_url: Option<String>,
}

// GET /v/:slug
// called when someone opens the share link
pub async fn get_video(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Json<VideoResponse> {

    let video = db::videos::get_by_slug(&state.db, &slug)
        .await
        .unwrap();

    Json(VideoResponse {
        status: format!("{:?}", video.status),
        manifest_url: video.hls_path
            .map(|p| state.storage.public_url(&p)),
    })
}

// GET /videos/:slug/*file
// serves the actual .ts and .m3u8 files to the browser
pub async fn serve_file(
    Path((slug, file)): Path<(String, String)>,
) -> impl IntoResponse {

    let file_path = format!("./storage/videos/{}/{}", slug, file);

    // read file from disk
    match tokio::fs::read(&file_path).await {
        Ok(contents) => {
            // figure out content type
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
        Err(_) => {
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}