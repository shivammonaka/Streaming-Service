use crate::models::video::{Video, VideoStatus};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

// INSERT a new video row — called when upload starts
pub async fn create(pool: &PgPool, slug: &str, original_path: &str) -> Result<Video> {
    let video = sqlx::query_as!(
        Video,
        r#"
        INSERT INTO videos (slug, status, original_path)
        VALUES ($1, 'Pending', $2)
        RETURNING id, slug, status as "status: VideoStatus", 
                  original_path, hls_path, created_at
        "#,
        slug,
        original_path
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

// SELECT by slug — called when someone opens the share link
pub async fn get_by_slug(pool: &PgPool, slug: &str) -> Result<Video> {
    let video = sqlx::query_as!(
        Video,
        r#"
        SELECT id, slug, status as "status: VideoStatus",
               original_path, hls_path, created_at
        FROM videos
        WHERE slug = $1
        "#,
        slug
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

// SELECT by id — called when polling status
pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Video> {
    let video = sqlx::query_as!(
        Video,
        r#"
        SELECT id, slug, status as "status: VideoStatus",
               original_path, hls_path, created_at
        FROM videos
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

// UPDATE status — called by ffmpeg when done
pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: VideoStatus,
    hls_path: Option<String>,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET status = $1, hls_path = $2
        WHERE id = $3
        "#,
        status as VideoStatus,
        hls_path,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}