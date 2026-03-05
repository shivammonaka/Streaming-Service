use async_trait::async_trait;
use anyhow::Result;

// This is the CONTRACT
// Any storage backend MUST implement these functions
#[async_trait]
pub trait StorageBackend: Send + Sync {
    // save the uploaded file to storage
    // returns the path where it was saved
    async fn save_file(&self, filename: &str, data: Vec<u8>) -> Result<String>;
    
    // get the folder path for a video's HLS output
    fn hls_output_path(&self, slug: &str) -> String;
    
    // get the public URL for a file
    fn public_url(&self, path: &str) -> String;
}

pub mod local;