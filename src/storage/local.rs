use super::StorageBackend;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct LocalStorage {
    pub base_path: String,  // e.g. "./storage"
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn save_file(&self, filename: &str, data: Vec<u8>) -> Result<String> {
        // create uploads folder if it doesn't exist
        let upload_dir = format!("{}/uploads", self.base_path);
        fs::create_dir_all(&upload_dir).await?;

        // save file to disk
        let file_path = format!("{}/{}", upload_dir, filename);
        fs::write(&file_path, data).await?;

        Ok(file_path)
    }

    fn hls_output_path(&self, slug: &str) -> String {
        // e.g. "./storage/videos/xK92mPqR"
        format!("{}/videos/{}", self.base_path, slug)
    }

    fn public_url(&self, path: &str) -> String {
        // just return the path as-is for local storage
        // for S3 this would return "https://cdn.example.com/..."
        path.to_string()
    }
}