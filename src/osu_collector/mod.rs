pub mod constants;
pub mod types;

use self::types::Collection;
use async_recursion::async_recursion;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use reqwest::{Client, Error};
use std::{fs, io::Cursor, path::Path};
use tokio::fs::File;

fn get_random_element<T>(array: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();
    array.choose(&mut rng)
}

#[derive(Debug)]
pub struct OsuCollector {
    api_url: &'static str,
    http_client: Client,
    progress_bar: ProgressBar,
}

impl Default for OsuCollector {
    fn default() -> Self {
        let progress_bar = ProgressBar::new(0);

        progress_bar.set_message("Downloading beatmaps progress");
        progress_bar.set_style(ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({per_sec}, {eta})").expect("Failed to set progress bar style")
                .progress_chars("#>-"));

        Self {
            api_url: "https://osucollector.com/api",
            http_client: Client::new(),
            progress_bar,
        }
    }
}

impl OsuCollector {
    #[allow(dead_code)]
    pub fn new(api_url: &'static str, http_client: Client) -> Self {
        Self {
            api_url,
            http_client,
            ..Default::default()
        }
    }

    pub async fn get_collection(&self, id: usize) -> Result<Collection, Error> {
        let url = format!("{}/collections/{id}", self.api_url);
        let response = self.http_client.get(&url).send().await?;
        let collection = response.json::<Collection>().await?;

        Ok(collection)
    }

    #[async_recursion]
    pub async fn download(&self, id: usize) {
        let mirror = get_random_element(&constants::DOWNLOAD_MIRROR).unwrap();
        let url = format!("{mirror}/{id}");
        let path = Path::new(constants::DOWNLOAD_PATH).join(format!("{id}.osz"));

        match fs::read_dir(constants::DOWNLOAD_PATH) {
            Ok(_) => {
                if let Ok(res) = self.http_client.get(&url).send().await {
                    if let Some(content_length) = res.content_length() {
                        let mut stream = res.bytes_stream();

                        self.progress_bar.set_length(content_length);

                        let mut file = File::create(&path).await.unwrap();

                        while let Some(chunk) = stream.next().await {
                            let chunk_data = chunk.unwrap();
                            self.progress_bar.inc(chunk_data.len() as u64);

                            let mut content = Cursor::new(chunk_data);
                            tokio::io::copy(&mut content, &mut file).await.unwrap();
                        }

                        log::info!("Downloaded: {}", path.display());
                        self.progress_bar.finish();
                    }

                    log::error!("Failed to download: {}", url);
                }
            }
            Err(_) => {
                fs::create_dir_all(constants::DOWNLOAD_PATH).unwrap();
                self.download(id).await;
            }
        }
    }
}
