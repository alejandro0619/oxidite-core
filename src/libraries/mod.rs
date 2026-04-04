use crate::downloader::Downloader;
use crate::models::OxiditeError;
use crate::models::{ProgressReport, piston_meta::PistonMeta};
use crate::sources::VerifiedFiledSource;
use futures::stream::{self, StreamExt};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn download_all<F>(
    meta: &PistonMeta,
    base_path: &Path,
    dl: &Downloader,
    on_progress: F,
) -> Result<(), OxiditeError>
where
    F: Fn(ProgressReport) + Send + Sync + 'static,
{
    let tasks: Vec<VerifiedFiledSource> = meta
        .libraries
        .iter()
        .filter(|lib| lib.is_compatible())
        .filter_map(|lib| {
            let artifact = &lib.downloads.artifact;
            let dest = base_path.join("libraries").join(&artifact.path);

            if !dest.exists() {
                Some(VerifiedFiledSource {
                    url: artifact.url.clone(),
                    dest,
                    expected_hash: artifact.sha1.clone(),
                })
            } else {
                None
            }
        })
        .collect();

    let total = tasks.len();
    if total == 0 {
        return Ok(());
    }

    let current_counter = Arc::new(AtomicUsize::new(0));
    let on_progress = Arc::new(on_progress);

    let results = stream::iter(tasks)
        .map(|source| {
            let downloader = dl;
            // Clone the Arc pointers for each task
            let counter = Arc::clone(&current_counter);
            let progress_cb = Arc::clone(&on_progress);

            async move {
                let res = downloader.download(source).await;

                // Once the download is complete, we increment the counter and report progress
                let prev = counter.fetch_add(1, Ordering::SeqCst);
                let current = prev + 1;

                // Report progress after each download completes
                progress_cb(ProgressReport {
                    current,
                    total,
                    message: format!("Downloading {} out of {}", current, total),
                });

                res
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    for res in results {
        res?;
    }

    Ok(())
}
