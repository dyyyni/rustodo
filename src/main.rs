mod app;
use crate::app::Rustodo;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Rustodo", native_options,
     Box::new(|_| Ok(Box::new(Rustodo::default()))));
}

