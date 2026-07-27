use crate::format::{format_large_number, format_percentage, format_price};
use crate::models::Coin;
use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};
use std::collections::HashSet;

pub fn show_coins_table(
    ui: &mut egui::Ui,
    filtered_coins: &[&Coin],
    favorite_coins: &HashSet<String>,
    clicked_coin_id: &mut Option<String>,
    favorite_clicked: &mut Option<String>,
) {
    TableBuilder::new(ui)
        .striped(true) // alternate row colors
        .resizable(true) // user can drag column borders
        .vscroll(true) // scroll when needed
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto()) // Favorite
        .column(Column::auto()) // Rank
        .column(Column::auto()) // Symbol
        .column(Column::remainder()) // Name
        .column(Column::auto()) // Price
        .column(Column::auto()) // 24h
        .column(Column::auto()) // Market Cap
        .column(Column::auto()) // Volume
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("");
            });
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
            body.rows(20.0, filtered_coins.len(), |mut row| {
                let coin = filtered_coins[row.index()];

                row.col(|ui| {
                    let is_favorite = favorite_coins.contains(&coin.id);

                    let icon = if is_favorite { "★" } else { "☆" };

                    let color = if is_favorite {
                        Color32::YELLOW
                    } else {
                        Color32::GRAY
                    };

                    if ui
                        .add(egui::Button::new(egui::RichText::new(icon).color(color)))
                        .clicked()
                    {
                        *favorite_clicked = Some(coin.id.clone());
                    }
                });

                row.col(|ui| {
                    ui.label(coin.market_cap_rank.to_string());
                });

                row.col(|ui| {
                    if ui.selectable_label(false, &coin.symbol).clicked() {
                        *clicked_coin_id = Some(coin.id.clone());
                    }
                });

                row.col(|ui| {
                    if ui.selectable_label(false, &coin.name).clicked() {
                        *clicked_coin_id = Some(coin.id.clone());
                    }
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
