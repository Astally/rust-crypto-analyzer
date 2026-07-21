use crate::models::Coin;
use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};
use thousands::Separable;

pub struct CryptoApp {
    coins: Vec<Coin>,
}

impl CryptoApp {
    pub fn new(coins: Vec<Coin>) -> Self {
        CryptoApp { coins }
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Top 10 Cryptocurrencies");
            });
            ui.add_space(10.0);

            // build professional table
            TableBuilder::new(ui)
                .striped(true) // alternate row colors
                .resizable(true) // user can drag column borders
                .vscroll(true) // scroll when needed
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto()) // Rank
                .column(Column::auto()) // Symbol
                .column(Column::remainder()) // Name
                .column(Column::auto()) // Price
                .column(Column::auto()) // 24h
                .column(Column::auto()) // Market Cap
                .column(Column::auto()) // Volume
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Rank");
                    });
                    header.col(|ui| {
                        ui.strong("Symbol");
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Price");
                    });
                    header.col(|ui| {
                        ui.strong("24h Change");
                    });
                    header.col(|ui| {
                        ui.strong("Market Cap");
                    });
                    header.col(|ui| {
                        ui.strong("Volume");
                    });
                })
                .body(|body| {
                    // populate rows
                    body.rows(20.0, self.coins.len(), |mut row| {
                        let coin = &self.coins[row.index()];

                        row.col(|ui| {
                            ui.label(coin.market_cap_rank.to_string());
                        });
                        row.col(|ui| {
                            ui.label(coin.symbol.to_uppercase());
                        });
                        row.col(|ui| {
                            ui.label(&coin.name);
                        });
                        row.col(|ui| {
                            ui.label(format_price(coin.current_price));
                        });

                        // colorize percentage
                        row.col(|ui| {
                            let text = format_percentage(coin.price_change_percentage_24h);
                            let color = if coin.price_change_percentage_24h > 0.0 {
                                Color32::LIGHT_GREEN
                            } else if coin.price_change_percentage_24h < 0.0 {
                                Color32::LIGHT_RED
                            } else {
                                Color32::WHITE
                            };
                            ui.label(egui::RichText::new(text).color(color));
                        });

                        row.col(|ui| {
                            ui.label(format_large_number(coin.market_cap));
                        });
                        row.col(|ui| {
                            ui.label(format_large_number(coin.total_volume));
                        });
                    });
                });
        });
    }
}

fn format_large_number(large_number: u64) -> String {
    let number: String;

    if large_number >= 1_000_000_000_000 {
        number = format!("{:.2} T", (large_number as f64) / 1_000_000_000_000.0);
    } else if large_number >= 1_000_000_000 {
        number = format!("{:.2} B", (large_number as f64) / 1_000_000_000.0);
    } else if large_number >= 1_000_000 {
        number = format!("{:.2} M", (large_number as f64) / 1_000_000.0);
    } else {
        number = large_number.to_string();
    }

    number
}

fn format_percentage(percent_number: f64) -> String {
    let number: String;
    if percent_number > 0.0 {
        number = format!("+{:.2}%", percent_number);
    } else {
        number = format!("{:.2}%", percent_number);
    }

    number
}

fn format_price(price: f64) -> String {
    return format!("${}", format!("{:.2}", price).separate_with_commas());
}
