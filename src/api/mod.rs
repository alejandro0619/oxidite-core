use crate::downloader::Downloader;
use crate::models::{Manifest, OxiditeError, PistonMeta};
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
    pub async fn get_manifest(&self) -> Result<Manifest, OxiditeError> {
        self.downloader.fetch(Sources::Manifest).await
    }

    pub async fn get_version_metadata(
        &self,
        hash: &str,
        id: &str,
    ) -> Result<PistonMeta, OxiditeError> {
        self.downloader
            .fetch(Sources::PistonMeta {
                hash: hash.to_string(),
                version: id.to_string(),
            })
            .await
    }
}
