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
            egui::Grid::new("coins_table").show(ui, |ui| {
                ui.heading("Rank");
                ui.heading("Symbol");
                ui.heading("Name");
                ui.heading("Price");
                ui.heading("24h %");
                ui.heading("Market Cap");
                ui.heading("Volume");

                ui.end_row();

                for coin in &self.coins {
                    ui.label(&coin.market_cap_rank.to_string());
                    ui.label(&coin.symbol);
                    ui.label(&coin.name);
                    ui.label(coin.current_price.to_string());
                    ui.label(coin.price_change_percentage_24h.to_string());
                    ui.label(format_large_number(coin.market_cap));
                    ui.label(format_large_number(coin.total_volume));

                    ui.separator();

                    ui.end_row();
                }
            });
        });
    }
}

fn format_large_number(large_number: u64) -> String {
    let number: String;

    if large_number >= 1_000_000_000_000 {
        number = format!("{:.2} T", (large_number as f64) / 1_000_000_000_000.0);
    } else if large_number >= 1_000_000_000 {
        number = format!("{:.2} B", (large_number as f64) / 1_000_000_000.0)
    } else if large_number >= 1_000_000 {
        number = format!("{:.2} M", (large_number as f64) / 1_000_000.0)
    } else {
        number = large_number.to_string()
    };

    number
}
