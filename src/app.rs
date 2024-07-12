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

impl Rustodo {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Henlo world.");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}

impl eframe::App for Rustodo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}