use crate::models::piston_meta::PistonMeta;
use crate::downloader::Downloader;
use crate::sources::FileSource;
use std::path::Path;

pub async fn download_all(
    meta: &PistonMeta, 
    base_path: &Path, 
    dl: &Downloader
) -> Result<(), Box<dyn std::error::Error>> {
    
    for lib in &meta.libraries {
        if lib.is_compatible() {
            let artifact = &lib.downloads.artifact;
            let dest = base_path.join("libraries").join(&artifact.path);

            if !dest.exists() {
                println!("📦 Descargando librería: {}", lib.name);
                
                let source = FileSource {
                    url: artifact.url.clone(),
                    dest,
                };

                dl.fetch(source).await?;
            }
        }
    }
    Ok(())
}