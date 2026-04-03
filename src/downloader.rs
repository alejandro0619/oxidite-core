use crate::sources::{AsUrl, DownloadTarget};
use reqwest::Client;
use std::time::Duration;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .user_agent("Oxidite-Launcher/1.0")
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn fetch<T>(&self, source: impl AsUrl) -> Result<T, Box<dyn std::error::Error>> 
    where T: serde::de::DeserializeOwned 
    {
        let bytes = self.client.get(source.as_url()).send().await?.bytes().await?;
        Ok(serde_json::from_slice::<T>(&bytes)?)
    }

    pub async fn download(&self, target: impl DownloadTarget) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(target.url()).send().await?;
        let bytes = response.bytes().await?;
        target.process(bytes).await
    }
}
