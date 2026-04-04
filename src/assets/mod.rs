use crate::dirs::MinecraftDirs;
use crate::downloader::Downloader;
use crate::models::OxiditeError;
use crate::models::assets::AssetIndex;
use crate::models::{ProgressReport, piston_meta::PistonMeta};
use crate::sources::Sources;
use crate::sources::VerifiedFiledSource;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn download_all<F>(
    meta: &PistonMeta,
    dirs: &MinecraftDirs,
    dl: &Downloader,
    on_progress: F,
) -> Result<(), OxiditeError>
where
    F: Fn(ProgressReport) + Send + Sync + 'static,
{
    // 1. Download the asset index JSON if it doesn't exist
    let index: AssetIndex = dl
        .fetch(Sources::AssetIndex(meta.asset_index.url.clone()))
        .await?;

    let index_path = dirs.asset_index_path(&meta.asset_index.id);
    if !index_path.exists() {
        let json_data = serde_json::to_string_pretty(&index)?;
        std::fs::write(&index_path, json_data)?;
    }

    let total = index.objects.len();
    let current_counter = Arc::new(AtomicUsize::new(0));
    let on_progress = Arc::new(on_progress);


    let tasks = futures::stream::iter(index.objects.into_iter().map(|(_name, object)| {
        let hash = object.hash.clone();
        let prefix = hash[..2].to_string();
        let dest = dirs.assets_dir().join("objects").join(&prefix).join(&hash);

        let url = format!(
            "https://resources.download.minecraft.net/{}/{}",
            prefix, hash
        );

        let downloader = dl;
        
        async move {
            if !dest.exists() {
                let source = VerifiedFiledSource {
                    url,
                    dest,
                    expected_hash: hash,
                };
                downloader.download(source).await
            } else {
                Ok(())
            }
        }
    }));

    // 4. Limit of concurrent downloads and progress tracking, this can be  adjusted through a config 
    let mut results = tasks.buffer_unordered(15);

    while let Some(result) = results.next().await {
        // Increment the counter atomically
        let prev = current_counter.fetch_add(1, Ordering::SeqCst);
        let current = prev + 1;

        // Emit the progress update
        on_progress(ProgressReport {
            current,
            total,
            message: format!("Sincronizando assets... ({} de {})", current, total),
        });

        if let Err(e) = result {
            // Optional to send the error through a channel or log it
            eprintln!("⚠️ Error on asset: {}", e);
        }
    }

    Ok(())
}