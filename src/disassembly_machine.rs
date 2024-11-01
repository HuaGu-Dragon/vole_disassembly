#[derive(Default)]
pub struct DisassemblyMachine {
    // --snip--
}

impl DisassemblyMachine {
    // --snip--
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for DisassemblyMachine {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
    // --snip--
}
