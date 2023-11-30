use crate::CollectorApp;
use eframe::egui;

impl eframe::App for CollectorApp {
	fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
		egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if self.downloading {
			if let Some(download) = &mut self.download {
				if let Some(beatmapsets) = download.ready() {
					self.downloading = false;
					self.collection_id.clear();
					println!("Downloaded {} beatmapsets", beatmapsets.len());
				}
			}
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.group(|ui| {
				ui.heading("Osu!Collector");
				ui.horizontal(|ui| {
					let collection_label = ui.label("Collection ID:");
					ui.text_edit_singleline(&mut self.collection_id)
						.labelled_by(collection_label.id)
						.on_hover_text("Enter collection ID");

					if self.downloading {
						ui.set_enabled(false);
						ui.button("Downloading...")
							.on_hover_text("Downloading collection...");

						ui.separator();

					// ui.add(
					//     egui::ProgressBar::new()
					//     )
					//                             );
					} else {
						if ui
							.button("Download")
							.on_hover_text("Download collection")
							.clicked()
						{
							let collection_id = self.collection_id.parse().unwrap();
							self.download = Some(CollectorApp::download_collection(collection_id));
							self.downloading = true;
						};
					}
				})
			});
		});

		ctx.request_repaint();
	}
}
