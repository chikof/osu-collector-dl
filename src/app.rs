use crate::defines::FONT_FAMILY;
use crate::defines::FONT_SIZE;
use eframe::epaint::FontId;
use osu_collector::types::Beatmapset;
use osu_collector::OsuCollector;
use poll_promise::Promise;

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct CollectorApp {
	pub collection_id: String,
	pub downloading: bool,
	pub rt: tokio::runtime::Runtime,
	pub download: Option<Promise<Vec<Beatmapset>>>,
}

impl Default for CollectorApp {
	fn default() -> Self {
		Self {
			collection_id: String::new(),
			downloading: false,
			rt: tokio::runtime::Builder::new_multi_thread()
				.enable_all()
				.build()
				.unwrap(),
			download: None,
		}
	}
}

impl CollectorApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		let ctx = &cc.egui_ctx;

		let mut style = (*ctx.style()).clone();
		let font = FontId {
			size: FONT_SIZE,
			family: FONT_FAMILY,
		};
		style.override_font_id = Some(font);
		ctx.set_style(style);

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		#[cfg(feature = "persistence")]
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}

	pub fn download_collection(id: usize) -> Promise<Vec<Beatmapset>> {
		Promise::spawn_async(async move {
			let client = OsuCollector::default();
			client.download_collection(id).await
		})
	}
}
