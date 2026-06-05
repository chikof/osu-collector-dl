use std::path::PathBuf;

use crate::CollectorApp;
use eframe::egui;

impl eframe::App for CollectorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.group(|ui| {
                ui.heading("Osu!Collector");
                ui.horizontal(|ui| {
                    let collection_label = ui.label("Collection ID:");
                    ui.text_edit_singleline(&mut self.collection_id)
                        .labelled_by(collection_label.id)
                        .on_hover_text("Enter collection ID");

                    ui.checkbox(&mut self.save_collection, "C")
                        .on_hover_text("Create collection");

                    if self.downloading {
                        ui.disable();
                        ui.button("...")
                            .on_hover_text("Downloading collection...")
                            .on_hover_cursor(egui::CursorIcon::Wait);

                        ui.separator();

                    // ui.add(
                    //     egui::ProgressBar::new()
                    //     )
                    //                             );
                    } else if ui
                        .button("Download")
                        .on_hover_text("Download collection")
                        .clicked()
                    {
                        let collection_id = self.collection_id.parse().unwrap();
                        self.download = Some(CollectorApp::download_collection(
                            collection_id,
                            self.collection_path.clone().map(PathBuf::from),
                        ));
                        self.downloading = true;
                    };
                })
            });

            ui.group(|ui| {
                ui.heading("Settings...");
                ui.horizontal(|ui| {
                    if ui
                        .button("collection.db")
                        .on_hover_text("Your collection.db file")
                        .clicked()
                        && let Some(path) = rfd::FileDialog::new().pick_file()
                    {
                        log::info!("CLICKED THE MOTHERFUCKING BUTTON!!!");
                        self.collection_path = Some(path.display().to_string());
                    }

                    if let Some(collection_path) = &self.collection_path {
                        ui.label(collection_path);
                    }
                })
            });

            ui.group(|ui| {
                ui.heading(format!("Collections ({}):", self.collections_size));

                for collection in &self.collections {
                    if let Some(name) = &collection.name {
                        ui.label(name);
                    }
                }
            });
        });
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.downloading
            && let Some(download) = &mut self.download
            && let Some(beatmapsets) = download.ready()
        {
            self.downloading = false;
            self.collection_id.clear();
            log::info!("Downloaded {} beatmapsets", beatmapsets.len());
        }

        if self.collection_path.is_some() && self.collections_size == 0 {
            self.collections = self.collections();
            self.collections_size = self.collections.len();
        }
    }
}
