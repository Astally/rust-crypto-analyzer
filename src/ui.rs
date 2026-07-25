use crate::client::get_market_data;
use crate::models::Coin;
use chrono::Local;
use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};
use thousands::Separable;
use tokio::sync::mpsc::{Receiver, Sender};

const DEFAULT_PER_PAGE: u32 = 10;

pub struct CryptoApp {
    coins: Vec<Coin>,
    per_page: u32,

    tx: Sender<Vec<Coin>>,
    rx: Receiver<Vec<Coin>>,

    last_updated: Option<chrono::DateTime<Local>>,
}

impl CryptoApp {
    pub fn new(coins: Vec<Coin>, tx: Sender<Vec<Coin>>, rx: Receiver<Vec<Coin>>) -> Self {
        CryptoApp {
            coins,
            per_page: DEFAULT_PER_PAGE,
            last_updated: Some(Local::now()),
            rx,
            tx,
        }
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(coins) = self.rx.try_recv() {
            self.coins = coins;
            self.last_updated = Some(Local::now());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Crypto Market");
            });

            //last update time
            if let Some(time) = self.last_updated {
                ui.label(
                    egui::RichText::new(format!(
                        "Last updated: {}",
                        time.format("%Y-%m-%d %H:%M:%S")
                    ))
                    .color(Color32::LIGHT_BLUE)
                    .small(),
                );
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Refresh").clicked() {
                    let tx = self.tx.clone();

                    let per_page = self.per_page;

                    tokio::spawn(async move {
                        let res = get_market_data(per_page).await;

                        match res {
                            Ok(coins) => {
                                if let Err(error) = tx.send(coins).await {
                                    eprintln!("Failed to send data: {error}");
                                }
                            }

                            Err(error) => {
                                eprintln!("Failed to refresh data: {error}");
                            }
                        }
                    });
                };
                egui::ComboBox::from_label("Coins")
                    .selected_text(self.per_page.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.per_page, 10, "10");
                        ui.selectable_value(&mut self.per_page, 20, "20");
                        ui.selectable_value(&mut self.per_page, 50, "50");
                        ui.selectable_value(&mut self.per_page, 100, "100");
                    });
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
                        row.col(|ui| match coin.price_change_percentage_24h {
                            Some(change) => {
                                let color = if change > 0.0 {
                                    Color32::LIGHT_GREEN
                                } else if change < 0.0 {
                                    Color32::LIGHT_RED
                                } else {
                                    Color32::WHITE
                                };

                                ui.label(
                                    egui::RichText::new(format_percentage(change)).color(color),
                                );
                            }

                            None => {
                                ui.label("-");
                            }
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

fn format_large_number(large_number: f64) -> String {
    let number: String;

    if large_number >= 1_000_000_000_000.0 {
        number = format!("{:.2} T", large_number / 1_000_000_000_000.0);
    } else if large_number >= 1_000_000_000.0 {
        number = format!("{:.2} B", large_number / 1_000_000_000.0);
    } else if large_number >= 1_000_000.0 {
        number = format!("{:.2} M", large_number / 1_000_000.0);
    } else {
        number = large_number.separate_with_commas();
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
