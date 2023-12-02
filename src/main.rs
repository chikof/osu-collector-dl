#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use crate::{
	app::CollectorApp,
	defines::{APP_NAME, WINDOW_HEIGHT, WINDOW_WIDTH},
};
use eframe::egui;

mod app;
mod defines;
mod gui;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT))
			// .with_always_on_top()
			.with_drag_and_drop(false)
			.with_decorations(true)
			.with_resizable(false)
			.with_transparent(true)
			.with_icon(utils::load_icon()),
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
}
