use std::path::PathBuf;

use crate::defines::FONT_FAMILY;
use crate::defines::FONT_SIZE;
use eframe::epaint::FontId;
use osu_collector::OsuCollector;
use osu_collector::osu::collection::Collection;
use osu_collector::types::Beatmapset;
use poll_promise::Promise;

pub struct CollectorApp {
    pub collection_id: String,
    pub downloading: bool,
    pub collection_path: Option<String>,
    pub save_collection: bool,
    pub collections_size: usize,
    pub collections: Vec<Collection>,
    pub download: Option<Promise<Vec<Beatmapset>>>,
}

impl Default for CollectorApp {
    fn default() -> Self {
        Self {
            collection_id: String::new(),
            downloading: false,
            collection_path: Some(
                "/home/chiko/Projects/osu-collector-dl/collection.db".to_string(),
            ),
            collections_size: 0,
            collections: Vec::new(),
            save_collection: false,
            download: None,
        }
    }
}

impl CollectorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        let mut style = (*ctx.global_style()).clone();
        let font = FontId {
            size: FONT_SIZE,
            family: FONT_FAMILY,
        };
        style.override_font_id = Some(font);
        ctx.set_global_style(style);

        Default::default()
    }

    pub fn download_collection(id: usize, path: Option<PathBuf>) -> Promise<Vec<Beatmapset>> {
        Promise::spawn_async(async move {
            let client = OsuCollector::default();
            client.download_collection(id, path).await
        })
    }

    pub fn collections(&self) -> Vec<Collection> {
        let client = OsuCollector::default();
        client.inspect_collection(self.collection_path.clone().map(PathBuf::from).unwrap())
    }
}
