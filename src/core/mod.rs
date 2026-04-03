use sha1::{Digest, Sha1};

use crate::launcher::LaunchSettings;
use crate::models::PistonMeta;
use crate::{api::Api, dirs::MinecraftDirs, sources::VerifiedFiledSource};

use std::path::PathBuf;
pub struct Oxidite {
    api: Api,
    dirs: MinecraftDirs,
}

impl Oxidite {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            api: Api::new(),
            dirs: MinecraftDirs::new(base_path),
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
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(crate::models::ProgressReport) + Send + Sync + Clone + 'static,
    {
        //1. Fetch manifest
        let manifest = self.api.get_manifest().await?;

        let entry = match version_id {
            Some(id) => manifest
                .find_version(id)
                .ok_or("Version not found in manifest")?,
            None => manifest
                .find_version(&manifest.latest.release)
                .ok_or("Latest version not found")?,
        };

        let hash = entry
            .get_hash()
            .ok_or("Could not extract hash from manifest URL")?;

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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());

        let mut child = command.spawn()?;
        child.wait()?;

        Ok(())
    }
}
