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
impl Api {

    pub async fn get_filtered_versions(&self, is_release: bool) -> Result<Vec<crate::models::Version>, OxiditeError> {
        let manifest = self.get_manifest().await?;
        let target_type = if is_release { "release" } else { "snapshot" };

        let filtered = manifest.versions
            .into_iter()
            .filter(|v| v.type_field == target_type) 
            .collect();

        Ok(filtered)
    }

    pub async fn find_version_by_id(&self, id: &str) -> Result<Option<crate::models::Version>, OxiditeError> {
        let manifest = self.get_manifest().await?;
        Ok(manifest.versions.into_iter().find(|v| v.id == id))
    }
}

impl Api {
    pub async fn check_java_compatibility(
        &self,
        hash: &str,
        id: &str,
        user_java_version: i64,
    ) -> Result<bool, OxiditeError> {

        let metadata = self.get_version_metadata(hash, id).await?;

        // Extract the required Java version from the metadata
        let required_java = metadata.java_version.major_version;

        // 3. Compatibility rules
        //  - If the required Java version is greater than the user's Java version, it's not compatible.
        //  - If the required Java version is less than or equal to the user's Java version, it's compatible.
        Ok(user_java_version >= required_java)
    }
}