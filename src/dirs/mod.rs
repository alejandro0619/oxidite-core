use std::path::{Path, PathBuf};
use std::fs;

pub struct MinecraftDirs {
    pub base: PathBuf,
}

impl MinecraftDirs {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    /// Creates the necessary directory structure for Minecraft if it doesn't already exist.
    pub fn create_all(&self, version: &str) -> std::io::Result<()> {
        fs::create_dir_all(self.versions_dir())?;
        fs::create_dir_all(self.version_specific_dir(version))?;
        fs::create_dir_all(self.libraries_dir())?;
        fs::create_dir_all(self.assets_dir().join("objects"))?;
        fs::create_dir_all(self.assets_dir().join("indexes"))?;
        Ok(())
    }

    // --- Some helpers for common paths ---

    pub fn versions_dir(&self) -> PathBuf {
        self.base.join("versions")
    }

    pub fn version_specific_dir(&self, version: &str) -> PathBuf {
        self.versions_dir().join(version)
    }

    /// Returns the path to the client JAR for a specific version.
    /// Example: .minecraft/versions/1.20.1/1.20.1.jar
    pub fn client_jar_path(&self, version: &str) -> PathBuf {
        self.version_specific_dir(version).join(format!("{}.jar", version))
    }

    pub fn libraries_dir(&self) -> PathBuf {
        self.base.join("libraries")
    }

    pub fn assets_dir(&self) -> PathBuf {
        self.base.join("assets")
    }

    pub fn asset_index_path(&self, id: &str) -> PathBuf {
        self.assets_dir().join("indexes").join(format!("{}.json", id))
    }
}