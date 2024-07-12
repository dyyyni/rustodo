use egui::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Rustodo {
    columns: Vec<Vec<String>>,
}

impl Default for Rustodo {
    fn default() -> Self {
        Self {
            columns: vec! [
                vec!["Item A", "Item B", "Item C", "Item D"],
                vec!["Item E", "Item F", "Item G"],
                vec!["Item H", "Item I", "Item J", "Item K"],
            ]
            .into_iter()
            .map(|v| v.into_iter().map(ToString::to_string).collect())
            .collect(),
        }
    }
}

impl eframe::App for Rustodo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("my_grid").show(ui, |ui| {
                ui.label("Row 1, Column 1");
                ui.label("Row 1, Column 2");
                ui.end_row();
                ui.label("Row 2, Column 1");
                ui.label("Row 2, Column 2");
            });
        });
   }
}