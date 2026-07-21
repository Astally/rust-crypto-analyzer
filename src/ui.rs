use std::format;

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
            ui.vertical_centered(|ui| {
                ui.heading("Top 10 Cryptocurrencies");
            });
            ui.add_space(10.0);

            egui::Grid::new("coins_table")
                .spacing([20.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    //headers
                    ui.strong("Rank");
                    ui.strong("Symbol");
                    ui.strong("Name");
                    ui.strong("Price");
                    ui.strong("24h %");
                    ui.strong("Market Cap");
                    ui.strong("Volume");

                    ui.end_row();

                    // data rows
                    for coin in &self.coins {
                        ui.label(&coin.market_cap_rank.to_string());
                        ui.label(&coin.symbol);
                        ui.label(&coin.name);

                        ui.label(format!("${:.2}", coin.current_price));

                        let percent_text = format_percentage(coin.price_change_percentage_24h);
                        if coin.price_change_percentage_24h > 0.0 {
                            ui.label(
                                egui::RichText::new(percent_text).color(egui::Color32::LIGHT_GREEN),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(percent_text).color(egui::Color32::LIGHT_RED),
                            );
                        }

                        ui.label(format_large_number(coin.market_cap));
                        ui.label(format_large_number(coin.total_volume));

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

fn format_percentage(percent_number: f64) -> String {
    let number = format!("{:.2}%", percent_number);
    number
}
