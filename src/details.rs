use crate::format::{format_large_number, format_percentage, format_price, format_timestamp};
use crate::models::{Chart, Coin, CoinDetails};
use eframe::egui::{self, Color32};
use egui::TextureHandle;
use egui_plot::{Line, Plot, PlotPoints};

pub fn show_coin_details(
    ui: &mut egui::Ui,
    coin: &Coin,
    details: &Option<CoinDetails>,
    coin_texture: &Option<TextureHandle>,
) {
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        if let Some(texture) = coin_texture {
            ui.image((texture.id(), egui::vec2(50.0, 50.0)));
        }

        ui.vertical(|ui| {
            ui.heading(&coin.name);
            ui.label(coin.symbol.to_uppercase());
        });
    });

    ui.add_space(20.0);

    ui.vertical(|ui| {
        ui.label("Homepage:");

        if let Some(details) = details {
            if let Some(homepage) = details.links.homepage.first() {
                if !homepage.is_empty() {
                    ui.hyperlink(homepage);
                } else {
                    ui.label("No homepage available.");
                }
            } else {
                ui.label("No homepage available.");
            }
        } else {
            ui.label("Loading homepage...");
        }
    });

    ui.add_space(15.0);

    ui.columns(2, |columns| {
        columns[0].group(|ui| {
            egui::Grid::new("market_data").show(ui, |ui| {
                ui.heading("Market Information");
                ui.end_row();

                ui.label("Rank");
                ui.strong(format!("#{}", coin.market_cap_rank));
                ui.end_row();

                ui.label("Price");
                ui.strong(format_price(coin.current_price));
                ui.end_row();

                ui.label("Market Cap");
                ui.strong(format_large_number(coin.market_cap));
                ui.end_row();

                ui.label("Volume");
                ui.strong(format_large_number(coin.total_volume));
                ui.end_row();

                ui.label("Last Update");
                ui.strong(coin.last_updated.split('T').next().unwrap_or("-"));
                ui.end_row();
            });
        });

        columns[1].group(|ui| {
            egui::Grid::new("price_data").show(ui, |ui| {
                ui.heading("Price Information");
                ui.end_row();

                ui.label("24h Change");
                match coin.price_change_percentage_24h {
                    Some(change) => {
                        let color = if change > 0.0 {
                            Color32::LIGHT_GREEN
                        } else if change < 0.0 {
                            Color32::LIGHT_RED
                        } else {
                            Color32::WHITE
                        };

                        ui.strong(egui::RichText::new(format_percentage(change)).color(color));
                    }

                    None => {
                        ui.strong("-");
                    }
                }
                ui.end_row();

                ui.label("High 24h");
                ui.strong(format_price(coin.high_24h.unwrap_or(0.0)));
                ui.end_row();

                ui.label("Low 24h");
                ui.strong(format_price(coin.low_24h.unwrap_or(0.0)));
                ui.end_row();

                ui.label("ATH");
                ui.strong(format_price(coin.ath));
                ui.end_row();

                ui.label("ATL");
                ui.strong(format_price(coin.atl));
                ui.end_row();
            });
        });
    });
}

pub fn show_price_chart(ui: &mut egui::Ui, chart: &Chart, coin_id: &str) {
    let points: PlotPoints = chart.prices.iter().map(|(t, p)| [*t, *p]).collect();

    let line = Line::new("Price", points)
        .color(Color32::from_rgb(38, 166, 154))
        .width(2.0_f32);

    Plot::new(format!("price_chart_{}", coin_id))
        .height(300.0)
        .x_axis_formatter(|ctx, _| {
            chrono::DateTime::from_timestamp(ctx.value as i64, 0)
                .map(|dt| dt.format("%H:%M").to_string())
                .unwrap_or_else(|| "-".to_string())
        })
        .y_axis_formatter(|ctx, _| format!("${:.2}", ctx.value))
        .label_formatter(|name, value| {
            format!("{}\n${:.2}\n{}", name, value.y, format_timestamp(value.x))
        })
        .show(ui, |plot_ui| plot_ui.line(line));
}
