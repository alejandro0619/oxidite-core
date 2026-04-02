const DEFAULT_VERSION: &str = "1.21.11";

pub struct CreateDirOptions {
    version: Option<String>,
}

impl CreateDirOptions {
    pub fn new(version: Option<String>) -> Self {
        Self { version }
    }
}
pub async fn create_dir(path: &std::path::Path, options: CreateDirOptions) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    // ---------- create a VERSION PATH

    // if a version is being provided, create a subdir for it.
    // if is not being provided, fallback to the last stable release, I'm hardcode 1.21.11 at the moment.

    if let Some(version) = options.version {
        let version_path = path.join("versions").join(version);
        std::fs::create_dir_all(version_path)?;
    } else {
        let version_path = path.join("versions").join(DEFAULT_VERSION);
        std::fs::create_dir_all(version_path)?;
    }

    // ---------- create a LIBRARIES PATH

    let library_path = path.join("libraries");
    std::fs::create_dir_all(library_path)?;

    // ---------- create an ASSETS PATH

    let asset_path = path.join("assets");
    std::fs::create_dir_all(asset_path)?;

    Ok(())
}
