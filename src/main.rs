#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use crate::{
	app::CollectorApp,
	defines::{APP_NAME, WINDOW_HEIGHT, WINDOW_WIDTH},
};
use eframe::egui;
use std::time::Duration;
use tokio::runtime::Runtime;

mod app;
mod defines;
mod gui;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
	// let client = crate::OsuCollector::default();

	let rt = Runtime::new().expect("Unable to create Runtime");

	// Enter the runtime so that `tokio::spawn` is available immediately.
	let _enter = rt.enter();

	// Execute the runtime in its own thread.
	// The future doesn't have to do anything. In this example, it just sleeps forever.
	std::thread::spawn(move || {
		rt.block_on(async {
			loop {
				tokio::time::sleep(Duration::from_secs(3600)).await;
			}
		})
	});

	let native_options = eframe::NativeOptions {
		// always_on_top: false,
		decorated: true,
		initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
		resizable: false,
		transparent: true,
		icon_data: Some(utils::load_icon()),
		..Default::default()
	};

	eframe::run_native(
		&format!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION")),
		native_options,
		Box::new(|cc| {
			cc.egui_ctx.set_visuals(egui::Visuals::dark());
			Box::new(CollectorApp::new(cc))
		}),
	)
	.unwrap();

	// client.download(1680649).await;
	// client.download(1413248).await;

	// log::info!("{:?}", client.get_collection(6600).await);
}
