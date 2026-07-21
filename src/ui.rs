use eframe::egui;

pub struct CryptoApp;

impl CryptoApp {
    pub fn new() -> Self {
        Self
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(150.0);

            ui.heading("Loading...");
        });
    }
}
