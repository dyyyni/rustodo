mod app;
use crate::app::Rustodo;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options,
     Box::new(|cc| Ok(Box::new(Rustodo::new(cc)))));
}

