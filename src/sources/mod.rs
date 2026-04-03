use std::path::PathBuf;
use async_trait::async_trait;
use sha1::{Sha1, Digest};


#[async_trait]
pub trait DownloadTarget {
    fn url(&self) -> String;
    async fn process(&self, bytes: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>>;
}

// Impl for files to be saved on disk
pub struct VerifiedFiledSource {
    pub url: String,
    pub dest: PathBuf,
    pub expected_hash: String,
}

#[async_trait]
impl DownloadTarget for VerifiedFiledSource {
    fn url(&self) -> String {
        self.url.clone()
    }

    async fn process(&self, bytes: bytes::Bytes) -> Result<(), Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        hasher.update(&bytes);

        let actual_hash = hex::encode(hasher.finalize());

        if actual_hash != self.expected_hash {
            return Err(format!("Hash mismatch for {}: expected {}, got {}", self.url, self.expected_hash, actual_hash).into());
        }

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
    AssetIndex(String)
}

impl AsUrl for Sources {
    fn as_url(&self) -> String {
        match self {
            Sources::Manifest => "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json".to_string(),
            Sources::PistonMeta { hash, version } => format!("https://piston-meta.mojang.com/v1/packages/{hash}/{version}.json"),
            Sources::AssetIndex(url) => url.clone(),
        }
    }
}