use std::path::PathBuf;
use async_trait::async_trait;


#[async_trait]
pub trait DownloadTarget {
    fn url(&self) -> String;
    async fn process(&self, bytes: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>>;
}

// Impl for files to be saved on disk
pub struct FileSource {
    pub url: String,
    pub dest: PathBuf,
}

#[async_trait]
impl DownloadTarget for FileSource {
    fn url(&self) -> String {
        self.url.clone()
    }

    async fn process(&self, bytes: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.dest, bytes)?;
        Ok(())
    }
}


pub trait AsUrl {
    fn as_url(&self) -> String;
}

pub enum Sources {
    Manifest,
    PistonMeta { hash: String, version: String },
}

impl AsUrl for Sources {
    fn as_url(&self) -> String {
        match self {
            Sources::Manifest => "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json".to_string(),
            Sources::PistonMeta { hash, version } => format!("https://piston-meta.mojang.com/v1/packages/{hash}/{version}.json"),
        }
    }
}