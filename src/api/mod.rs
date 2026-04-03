use crate::downloader::Downloader;
use crate::models::{Manifest, PistonMeta};
use crate::sources::Sources;

pub struct Api {
    pub downloader: Downloader,
}

impl Api {
    pub fn new() -> Self {
        Self {
            downloader: Downloader::new(),
        }
    }
}

impl Api {
    pub async fn get_manifest(&self) -> Result<Manifest, Box<dyn std::error::Error>> {
        self.downloader.fetch(Sources::Manifest).await
    }

    pub async fn get_version_metadata(
        &self,
        hash: &str,
        id: &str,
    ) -> Result<PistonMeta, Box<dyn std::error::Error>> {
        self.downloader
            .fetch(Sources::PistonMeta {
                hash: hash.to_string(),
                version: id.to_string(),
            })
            .await
    }
}
