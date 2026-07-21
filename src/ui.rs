use crate::models::Coin;
use eframe::egui;

pub struct CryptoApp {
    coins: Vec<Coin>,
}

impl CryptoApp {
    pub fn new(coins: Vec<Coin>) -> Self {
        CryptoApp { coins: coins }
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for coin in &self.coins {
                ui.label(format!("{} : {}", &coin.name, &coin.current_price));
            }
        });
    }
}
