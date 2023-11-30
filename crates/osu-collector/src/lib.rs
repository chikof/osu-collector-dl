pub mod constants;
pub mod types;

// use core::cmp::min;
use self::types::{Beatmapset, Collection};
use async_recursion::async_recursion;
use futures_util::StreamExt;
use rand::Rng;
use reqwest::{Client, Error};
use std::{fs::File, io::Write, path::Path};
use tokio::fs;

fn get_random_element<T>(array: &[T]) -> Option<&T> {
	let index = rand::thread_rng().gen_range(0..array.len());
	array.get(index)
}

#[derive(Debug, Clone)]
pub struct OsuCollector {
	api_url: &'static str,
	http_client: Client,
	pub version: String,
}

impl Default for OsuCollector {
	fn default() -> Self {
		Self {
			api_url: "https://osucollector.com/api",
			http_client: Client::new(),
			version: Self::version(),
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

	pub async fn download_collection(&self, id: usize) -> Vec<Beatmapset> {
		let collection = self.get_collection(id).await.unwrap();
		let mut new_beatmapsets: Vec<Beatmapset> = Vec::new();

		for beatmap in &collection.beatmapsets {
			if !self.download(beatmap.id.try_into().unwrap()).await {
				continue;
			}

			new_beatmapsets.push(beatmap.clone());
		}

		new_beatmapsets
	}

	#[async_recursion]
	async fn download(&self, id: usize) -> bool {
		let mirror = get_random_element(&constants::DOWNLOAD_MIRROR).unwrap();
		let url = format!("{mirror}/{id}");
		let path = Path::new(constants::DOWNLOAD_PATH).join(format!("{id}.osz"));

		return match fs::try_exists(constants::DOWNLOAD_PATH).await {
			Ok(_) => self.download_file(path.display().to_string(), url).await,
			Err(_) => {
				tokio::fs::create_dir_all(constants::DOWNLOAD_PATH)
					.await
					.unwrap();
				self.download(id).await
			}
		};
	}

	async fn download_file(&self, filename: String, url: String) -> bool {
		let res = self
			.http_client
			.get(url.clone())
			.send()
			.await
			.or(Err(format!("Failed to download file from {}", &url)))
			.unwrap();

		let total_size = res
			.content_length()
			.ok_or(format!("Failed to get content length from '{}'", &url))
			.unwrap();
		println!("Downloading {} bytes, {}", total_size, url);

		if total_size <= 1000 {
			println!("File too small, skipping");
			return false;
		}

		// download chunks
		let mut file = File::create(filename.clone())
			.or(Err(format!("Failed to create file '{}'", filename)))
			.unwrap();
		// let mut downloaded: u64 = 0;
		let mut stream = res.bytes_stream();

		while let Some(item) = stream.next().await {
			let chunk = item
				.or(Err(format!("Error while downloading file from {}", url)))
				.unwrap();
			file.write_all(&chunk)
				.or(Err(format!("Error while writing to file")))
				.unwrap();
			// let new = min(downloaded + (chunk.len() as u64), total_size);
			// downloaded = new;
			// println!("{}", ((new as f64/total_size as f64) as f64 * 10.0).round() / 10.0 );
		}

		return true;
	}

	pub fn version() -> String {
		format!("v{}", env!("CARGO_PKG_VERSION"))
	}
}
