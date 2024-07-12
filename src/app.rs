#[derive(Default)]
pub struct Rustodo {
    name: String,
    age: u32,
}

impl Rustodo {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            name: String::new(),
            age: 0,
        }
    }
}

impl eframe::App for Rustodo {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name);
           });
           ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
           if ui.button("Increment").clicked() {
            self.age += 1;
           }
           ui.label(format!("Hello {}, age {}", self.name, self.age));
       });
   }
}