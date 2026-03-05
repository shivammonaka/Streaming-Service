use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;
use nanoid::nanoid;
use crate::AppState;
use crate::db;
use crate::services::transcode;
#[derive(Serialize)]
pub struct UploadResponse {
    pub video_id: String,
    pub slug: String,
    pub share_url: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub manifest_url: Option<String>,
}

// POST /api/videos
pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {

    // 1. read file from request
    let mut file_data: Vec<u8> = Vec::new();
    let mut filename = String::from("video.mp4");

    while let Some(field) = multipart.next_field().await.unwrap() {
        filename = field.file_name()
            .unwrap_or("video.mp4")
            .to_string();
        file_data = field.bytes().await.unwrap().to_vec();
    }

    // 2. generate slug
    let slug = nanoid!(8);

    // 3. save file to disk
    let saved_path = state.storage
        .save_file(&filename, file_data)
        .await
        .unwrap();

    // 4. save to database
    let video = db::videos::create(&state.db, &slug, &saved_path)
        .await
        .unwrap();

    // 5. get output path for HLS chunks
    let hls_path = state.storage.hls_output_path(&slug);

    // 6. trigger ffmpeg in background (dont wait for it)
    let db_pool = state.db.clone();
    let video_id = video.id;
    let saved_path_clone = saved_path.clone();
    tokio::spawn(async move {
        transcode::run(video_id, &saved_path_clone, &hls_path, &db_pool).await;
    });

    // 7. return response immediately, dont wait for ffmpeg
    Json(UploadResponse {
        video_id: video.id.to_string(),
        slug: slug.clone(),
        share_url: format!("http://localhost:5173/v/{}", slug),
    })
}

// GET /api/videos/:id/status
pub async fn status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<StatusResponse> {

    let id = Uuid::parse_str(&id).unwrap();
    let video = db::videos::get_by_id(&state.db, id)
        .await
        .unwrap();

    Json(StatusResponse {
        status: format!("{:?}", video.status),
        manifest_url: video.hls_path
            .map(|p| format!("http://localhost:3000/{}", p)),
    })
}