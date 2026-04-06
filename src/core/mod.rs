use sha1::{Digest, Sha1};

use crate::launcher::LaunchSettings;
use crate::models::PistonMeta;
use crate::{api::Api, dirs::MinecraftDirs, sources::VerifiedFiledSource};
use crate::models::internal::{OxiditeConfig, OxiditeError};
use crate::instance::GameInstance;
pub struct Oxidite {
    pub config: OxiditeConfig,
    api: Api,
    dirs: MinecraftDirs,
}

impl Oxidite {
    pub fn new(config: OxiditeConfig) -> Self {
        let dirs = MinecraftDirs::new(config.base_path.clone());
        Self {
            api: Api::new(),
            dirs,
            config,
        }
    }
}

impl Oxidite {
    pub async fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let manifest = self.api.get_manifest().await?;

        Ok(manifest.latest.release)
    }

        pub async fn prepare_version<F>(
            &self,
            version_id: Option<&str>,
            on_progress: F,
        ) -> Result<(), OxiditeError>
        where
            F: Fn(crate::models::ProgressReport) + Send + Sync + Clone + 'static,
        {
            //1. Fetch manifest
            let manifest = self.api.get_manifest().await?;

            let entry = match version_id {
                Some(id) => manifest
                    .find_version(id)
                    .ok_or(OxiditeError::VersionNotFound(id.to_string()))?,
                None => manifest
                    .find_version(&manifest.latest.release)
                    .ok_or(OxiditeError::VersionNotFound(manifest.latest.release.to_string()))?,
            };

            let hash = entry
                .get_hash()
                .ok_or(OxiditeError::MetadataError("Could not extract hash from manifest URL".into()))?;

            // 2. Fetch version metadata
            let meta = self.api.get_version_metadata(&hash, &entry.id).await?;

            // 3. Create directories
            self.dirs.create_all(&entry.id)?;

            // 4. Download client jar
            self.download_client(&meta).await?;

            // 5. Download libraries
            crate::libraries::download_all(
                &meta,
                &self.dirs.base,
                &self.api.downloader,
                on_progress.clone(),
            )
            .await?;

            // 6. Download assets
            crate::assets::download_all(&meta, &self.dirs, &self.api.downloader, on_progress.clone())
                .await?;

            Ok(())
        }

}

impl Oxidite {
    pub async fn download_client(
        &self,
        meta: &PistonMeta,
    ) -> Result<(), OxiditeError> {
        let client_path = self.dirs.client_jar_path(&meta.id);
        let expected_hash = &meta.downloads.client.sha1;

        if client_path.exists() {
            let bytes = std::fs::read(&client_path)?;
            let mut hasher = Sha1::new();
            hasher.update(&bytes);
            let current_hash = hex::encode(hasher.finalize());

            if &current_hash == expected_hash {
                return Ok(()); // the file is already correct, no need to download
            }
        }

        let target = VerifiedFiledSource {
            url: meta.downloads.client.url.clone(),
            dest: client_path,
            expected_hash: expected_hash.clone(),
        };

        self.api.downloader.download(target).await?;

        Ok(())
    }
}

impl Oxidite {
    pub async fn launch(
        &self,
        version_id: &str,
        settings: LaunchSettings,
    ) -> Result<GameInstance, Box<dyn std::error::Error>> {
        let manifest = self.api.get_manifest().await?;
        let entry = manifest
            .find_version(version_id)
            .ok_or("Version not found")?;
        
        let hash = entry.get_hash().ok_or("Hash not found")?;
        let meta = self.api.get_version_metadata(&hash, version_id).await?;
        
        let mut command = settings.create_command(&meta, &self.dirs)?;
        command.current_dir(&self.dirs.base);
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::inherit());

        // Spwn the process
        let child = command.spawn()?;

        // Create the GameInstance struct with the child process and return it immediately
        

        Ok(GameInstance::new(version_id.to_string(), settings.username.clone(), child))
    }
}
impl Oxidite {
    /// Obtains all versions Releases or Snapshots
    pub async fn get_versions(&self, releases_only: bool) -> Result<Vec<crate::models::Version>, OxiditeError> {
        self.api.get_filtered_versions(releases_only).await
    }

    /// Look for a specific version by its ID (e.g. "1.20.1") and return its metadata if found
    pub async fn find_version(&self, id: &str) -> Result<Option<crate::models::Version>, OxiditeError> {
        self.api.find_version_by_id(id).await
    }

    /// check if the given Java major version (e.g. 17) is compatible with the Minecraft version specified by version_id (e.g. "1.20.1").
    pub async fn check_compatibility(&self, version_id: &str, user_java_major: i64) -> Result<bool, OxiditeError> {

        let version = self.find_version(version_id).await?
            .ok_or_else(|| OxiditeError::VersionNotFound(version_id.to_string()))?;

        let hash = version.get_hash()
            .ok_or_else(|| OxiditeError::MetadataError("No hash found in manifest".into()))?;

        self.api.check_java_compatibility(&hash, &version.id, user_java_major).await
    }

    /// Utility method to directly get the required Java major version for a given Minecraft version ID. Returns an error if the version is not found or if there's an issue with the metadata.
    pub async fn get_required_java_version(&self, version_id: &str) -> Result<i64, OxiditeError> {
        let version = self.find_version(version_id).await?
            .ok_or_else(|| OxiditeError::VersionNotFound(version_id.to_string()))?;
        
        let hash = version.get_hash()
            .ok_or_else(|| OxiditeError::MetadataError("Hash missing".into()))?;

        let meta = self.api.get_version_metadata(&hash, &version.id).await?;
        Ok(meta.java_version.major_version)
    }
}