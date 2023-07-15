pub mod constants;
pub mod types;

use std::fs;

use self::types::Collection;
use rand::seq::SliceRandom;
use reqwest::{Client, Error};

fn get_random_element<T>(array: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();
    array.choose(&mut rng)
}

#[derive(Debug)]
pub struct OsuCollector {
    api_url: &'static str,
    http_client: Client,
}

impl Default for OsuCollector {
    fn default() -> Self {
        Self {
            api_url: "https://osucollector.com/api",
            http_client: Client::new(),
        }
    }
}

impl OsuCollector {
    #[allow(dead_code)]
    pub fn new(api_url: &'static str, http_client: Client) -> Self {
        Self {
            api_url,
            http_client,
        }
    }

    pub async fn get_collection(&self, id: usize) -> Result<Collection, Error> {
        let url = format!("{}/collections/{id}", self.api_url);
        let response = self.http_client.get(&url).send().await?;
        let collection = response.json::<Collection>().await?;

        Ok(collection)
    }

    pub async fn download(&self, id: usize) -> () {
        let mirror = get_random_element(&constants::DOWNLOAD_MIRROR).unwrap();
        let url = format!("{mirror}/{id}");
        let file_name = format!("{}/{}.osz", constants::DOWNLOAD_PATH, id);

        match fs::create_dir_all(constants::DOWNLOAD_PATH) {
            Ok(_) => {
                fs::write(
                    file_name,
                    self.http_client
                        .get(&url)
                        .send()
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap(),
                )
                .unwrap();
            }
            Err(e) => panic!("Error creating directory: {}", e),
        }
    }
}
