use crate::client::get_market_data;
use crate::models::Coin;
use chrono::Local;
use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};
use std::cmp::Ordering;
use thousands::Separable;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CryptoApp {
    coins: Vec<Coin>,
    per_page: u32,
    last_updated: chrono::DateTime<Local>,
    search_query: String,
    sort_by: SortBy,
    sort_order: SortOrder,

    tx: Sender<anyhow::Result<Vec<Coin>>>,
    rx: Receiver<anyhow::Result<Vec<Coin>>>,

    error_message: Option<String>,
}

#[derive(Default, PartialEq)]
enum SortBy {
    #[default]
    Rank,
    Price,
    MarketCap,
    Volume,
    Change24h,
}

#[derive(Default, PartialEq)]
enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

impl CryptoApp {
    pub fn new(
        coins: Vec<Coin>,
        tx: Sender<anyhow::Result<Vec<Coin>>>,
        rx: Receiver<anyhow::Result<Vec<Coin>>>,
    ) -> Self {
        CryptoApp {
            coins,
            per_page: 10,
            last_updated: Local::now(),
            search_query: String::new(),
            sort_by: SortBy::default(),
            sort_order: SortOrder::default(),
            rx,
            tx,
            error_message: None,
        }
    }

    fn show_dashboard(&mut self, ui: &mut egui::Ui) {
        if let Some(error) = &self.error_message {
            ui.colored_label(Color32::RED, format!("Error: {error}"));
            ui.add_space(5.0);
        }

        ui.vertical_centered(|ui| {
            ui.heading("Crypto Market");
        });

        //last update time
        ui.label(
            egui::RichText::new(format!(
                "Last updated: {}",
                self.last_updated.format("%Y-%m-%d %H:%M:%S")
            ))
            .color(Color32::LIGHT_BLUE)
            .small(),
        );

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Refresh").clicked() {
                let tx = self.tx.clone();

                let per_page = self.per_page;

                tokio::spawn(async move {
                    let result = get_market_data(per_page).await;

                    if let Err(error) = tx.send(result).await {
                        eprintln!("Failed to send result: {error}");
                    }
                });
            };

            // number of coins box
            egui::ComboBox::from_label("Coins")
                .selected_text(self.per_page.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.per_page, 10, "10");
                    ui.selectable_value(&mut self.per_page, 20, "20");
                    ui.selectable_value(&mut self.per_page, 50, "50");
                    ui.selectable_value(&mut self.per_page, 100, "100");
                });

            // sort box
            egui::ComboBox::from_label("Sort")
                .selected_text(match self.sort_by {
                    SortBy::Rank => "Rank",
                    SortBy::Price => "Price",
                    SortBy::MarketCap => "Market Cap",
                    SortBy::Volume => "Volume",
                    SortBy::Change24h => "24h Change",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort_by, SortBy::Rank, "Rank");
                    ui.selectable_value(&mut self.sort_by, SortBy::Price, "Price");
                    ui.selectable_value(&mut self.sort_by, SortBy::MarketCap, "Market Cap");
                    ui.selectable_value(&mut self.sort_by, SortBy::Volume, "Volume");
                    ui.selectable_value(&mut self.sort_by, SortBy::Change24h, "24h Change");
                });

            // sort order box
            egui::ComboBox::from_id_salt("sort_order")
                .selected_text(match self.sort_order {
                    SortOrder::Ascending => "Ascending",
                    SortOrder::Descending => "Descending",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort_order, SortOrder::Ascending, "Ascending");

                    ui.selectable_value(&mut self.sort_order, SortOrder::Descending, "Descending");
                });

            // search box
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search by name or symbol...")
                    .desired_width(200.0),
            );
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
                let filtered_coins = self.get_filtered_coins();

                // populate rows
                body.rows(20.0, filtered_coins.len(), |mut row| {
                    let coin = filtered_coins[row.index()];

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

                            ui.label(egui::RichText::new(format_percentage(change)).color(color));
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
    }

    fn get_filtered_coins(&self) -> Vec<&Coin> {
        let query = self.search_query.trim().to_lowercase();

        let mut coins: Vec<&Coin> = self
            .coins
            .iter()
            .filter(|coin| {
                query.is_empty()
                    || coin.name.to_lowercase().contains(&query)
                    || coin.symbol.to_lowercase().contains(&query)
            })
            .collect();

        match (&self.sort_by, &self.sort_order) {
            (SortBy::Rank, SortOrder::Ascending) => {
                coins.sort_by(|a, b| a.market_cap_rank.cmp(&b.market_cap_rank));
            }

            (SortBy::Rank, SortOrder::Descending) => {
                coins.sort_by(|a, b| b.market_cap_rank.cmp(&a.market_cap_rank));
            }

            (SortBy::Price, SortOrder::Ascending) => {
                coins.sort_by(|a, b| {
                    b.current_price
                        .partial_cmp(&a.current_price)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::Price, SortOrder::Descending) => {
                coins.sort_by(|a, b| {
                    a.current_price
                        .partial_cmp(&b.current_price)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::MarketCap, SortOrder::Ascending) => {
                coins.sort_by(|a, b| {
                    b.market_cap
                        .partial_cmp(&a.market_cap)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::MarketCap, SortOrder::Descending) => {
                coins.sort_by(|a, b| {
                    a.market_cap
                        .partial_cmp(&b.market_cap)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::Volume, SortOrder::Ascending) => {
                coins.sort_by(|a, b| {
                    b.total_volume
                        .partial_cmp(&a.total_volume)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::Volume, SortOrder::Descending) => {
                coins.sort_by(|a, b| {
                    a.total_volume
                        .partial_cmp(&b.total_volume)
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::Change24h, SortOrder::Ascending) => {
                coins.sort_by(|a, b| {
                    b.price_change_percentage_24h
                        .unwrap_or(0.0)
                        .partial_cmp(&a.price_change_percentage_24h.unwrap_or(0.0))
                        .unwrap_or(Ordering::Equal)
                });
            }

            (SortBy::Change24h, SortOrder::Descending) => {
                coins.sort_by(|a, b| {
                    a.price_change_percentage_24h
                        .unwrap_or(0.0)
                        .partial_cmp(&b.price_change_percentage_24h.unwrap_or(0.0))
                        .unwrap_or(Ordering::Equal)
                });
            }
        }

        coins
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.rx.try_recv() {
            match result {
                Ok(coins) => {
                    self.coins = coins;
                    self.last_updated = Local::now();
                    self.error_message = None;
                }

                Err(error) => {
                    self.error_message = Some(error.to_string());
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_dashboard(ui);
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
