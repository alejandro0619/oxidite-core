use sha1::{Digest, Sha1};

use crate::launcher::LaunchSettings;
use crate::models::PistonMeta;
use crate::{api::Api, dirs::MinecraftDirs, sources::VerifiedFiledSource};
use crate::models::internal::{OxiditeConfig, OxiditeError};

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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = self.api.get_manifest().await?;
        let entry = manifest
            .find_version(version_id)
            .ok_or("Version not found")?;
        let hash = entry.get_hash().ok_or("Hash not found")?;
        let meta = self.api.get_version_metadata(&hash, version_id).await?;

        let mut command = settings.create_command(&meta, &self.dirs)?;
        command.current_dir(&self.dirs.base); // So the logs, screenshot, crash reports are generated in the correct place.
        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());

        let mut child = command.spawn()?;
        child.wait()?;

        Ok(())
    }
}
