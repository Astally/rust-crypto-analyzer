use crate::client::get_chart_data;
use crate::client::get_link_data;
use crate::client::get_market_data;
use crate::dashboard::show_coins_table;
use crate::details::{show_coin_details, show_price_chart};
use crate::filter;
use crate::models::{Chart, Coin, CoinDetails};
use crate::storage::{load_favorites, save_favorites};
use chrono::Local;
use eframe::egui::{self, Color32};
use std::collections::HashSet;
use std::time::Instant;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CryptoApp {
    coins: Vec<Coin>,
    per_page: u32,
    last_updated: chrono::DateTime<Local>,
    search_query: String,
    sort_by: SortBy,
    sort_order: SortOrder,

    selected_coin_id: Option<String>,
    coin_details: Option<CoinDetails>,
    chart_data: Option<Chart>,
    current_screen: AppScreen,

    favorite_coins: HashSet<String>,
    show_filter: CoinFilter,

    tx: Sender<anyhow::Result<Vec<Coin>>>,
    rx: Receiver<anyhow::Result<Vec<Coin>>>,

    details_tx: Sender<anyhow::Result<CoinDetails>>,
    details_rx: Receiver<anyhow::Result<CoinDetails>>,

    chart_tx: Sender<anyhow::Result<Chart>>,
    chart_rx: Receiver<anyhow::Result<Chart>>,

    error_message: Option<String>,
    error_time: Option<chrono::DateTime<Local>>,

    settings: Settings,
    show_settings: bool,
    is_refreshing: bool,
    last_refresh: Instant,
}

#[derive(Default)]
pub enum AppScreen {
    #[default]
    Dashboard,
    CoinDetails,
}

#[derive(Default, PartialEq)]
pub enum CoinFilter {
    #[default]
    All,
    Favorites,
}

#[derive(Default, PartialEq)]
pub enum SortBy {
    #[default]
    Rank,
    Price,
    MarketCap,
    Volume,
    Change24h,
}

#[derive(Default, PartialEq)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

pub struct Settings {
    pub auto_refresh: bool,
    pub refresh_interval: u64,
}

impl CryptoApp {
    pub fn new(
        coins: Vec<Coin>,
        tx: Sender<anyhow::Result<Vec<Coin>>>,
        rx: Receiver<anyhow::Result<Vec<Coin>>>,
        details_tx: Sender<anyhow::Result<CoinDetails>>,
        details_rx: Receiver<anyhow::Result<CoinDetails>>,
        chart_tx: Sender<anyhow::Result<Chart>>,
        chart_rx: Receiver<anyhow::Result<Chart>>,
    ) -> Self {
        let mut error_message = None;
        let mut error_time = None;

        let favorite_coins = match load_favorites() {
            Ok(favorites) => favorites,

            Err(error) => {
                eprintln!("Failed to load favorites: {error}");

                error_message = Some(error.to_string());
                error_time = Some(Local::now());

                HashSet::new()
            }
        };

        CryptoApp {
            coins,
            per_page: 10,
            last_updated: Local::now(),
            search_query: String::new(),
            sort_by: SortBy::default(),
            sort_order: SortOrder::default(),
            selected_coin_id: None,
            coin_details: None,
            chart_data: None,
            current_screen: AppScreen::default(),
            favorite_coins: favorite_coins,
            show_filter: CoinFilter::default(),
            error_time,
            rx,
            tx,
            details_rx,
            details_tx,
            chart_tx,
            chart_rx,
            error_message,
            settings: Settings {
                auto_refresh: false,
                refresh_interval: 10,
            },
            show_settings: false,
            is_refreshing: false,
            last_refresh: Instant::now(),
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
            if ui.button("Refresh").clicked() && !self.is_refreshing {
                self.is_refreshing = true;

                let tx = self.tx.clone();

                let per_page = self.per_page;

                tokio::spawn(async move {
                    let result = get_market_data(per_page).await;

                    if let Err(error) = tx.send(result).await {
                        eprintln!("Failed to send result: {error}");
                    }
                });

                self.last_refresh = Instant::now();
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

            // show all and favorite
            egui::ComboBox::from_id_salt("Show")
                .selected_text(match self.show_filter {
                    CoinFilter::All => "All Coins",
                    CoinFilter::Favorites => "Favorites",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.show_filter, CoinFilter::All, "All Coins");

                    ui.selectable_value(&mut self.show_filter, CoinFilter::Favorites, "Favorites");
                });

            // search box
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search by name or symbol...")
                    .desired_width(180.0),
            );

            // setting
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙").on_hover_text("Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        ui.add_space(10.0);

        let mut clicked_coin_id: Option<String> = None;
        let mut favorite_clicked: Option<String> = None;

        let filtered_coins = self.get_filtered_coins();

        let favorite_count = self.favorite_coins.len();

        if filtered_coins.is_empty() {
            ui.vertical_centered(|ui| {
                ui.heading("No coins found");
                ui.add_space(10.0);

                match self.show_filter {
                    CoinFilter::Favorites => {
                        if favorite_count == 0 {
                            ui.label("You haven't added any favorite coins yet.");
                        } else {
                            ui.label("No favorite coins match your search.");
                        }
                    }

                    CoinFilter::All => {
                        ui.label("No coins match your search.");
                    }
                }
            });

            return;
        }

        // build professional table
        show_coins_table(
            ui,
            &filtered_coins,
            &self.favorite_coins,
            &mut clicked_coin_id,
            &mut favorite_clicked,
        );

        if let Some(id) = clicked_coin_id {
            self.coin_details = None;
            self.chart_data = None;

            let tx = self.details_tx.clone();
            let coin_id = id.clone();

            tokio::spawn(async move {
                let result = get_link_data(&coin_id).await;

                if let Err(error) = tx.send(result).await {
                    eprintln!("Failed to send result: {error}");
                }
            });

            let tx = self.chart_tx.clone();
            let coin_id = id.clone();

            tokio::spawn(async move {
                let result = get_chart_data(&coin_id).await;

                if let Err(error) = tx.send(result).await {
                    eprintln!("Failed to send result: {error}");
                }
            });

            self.selected_coin_id = Some(id);
            self.current_screen = AppScreen::CoinDetails;
        }

        // favorite button
        if let Some(id) = favorite_clicked {
            if self.favorite_coins.contains(&id) {
                self.favorite_coins.remove(&id);
            } else {
                self.favorite_coins.insert(id);
            }

            // save favorite JSON
            if let Err(error) = save_favorites(&self.favorite_coins) {
                self.error_message = Some(error.to_string());
                self.error_time = Some(Local::now());
            }
        }
    }

    fn get_filtered_coins(&self) -> Vec<&Coin> {
        filter::get_filtered_coins(
            &self.coins,
            &self.search_query,
            &self.show_filter,
            &self.favorite_coins,
            &self.sort_by,
            &self.sort_order,
        )
    }

    fn show_coin_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("Back").clicked() {
            self.current_screen = AppScreen::Dashboard;
        }

        let Some(id) = &self.selected_coin_id else {
            ui.label("No coin selected.");
            return;
        };

        let Some(coin) = self.coins.iter().find(|coin| &coin.id == id) else {
            ui.label("Coin not found.");
            return;
        };

        show_coin_details(ui, coin, &self.coin_details);

        if let Some(chart) = &self.chart_data {
            ui.add_space(20.0);
            ui.heading("24h Price Chart");
            show_price_chart(ui, chart, &coin.id);
        } else {
            ui.label("Loading chart...");
        }
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.rx.try_recv() {
            self.is_refreshing = false;

            match result {
                Ok(coins) => {
                    self.coins = coins;
                    self.last_updated = Local::now();

                    self.error_message = None;
                    self.error_time = None;
                }

                Err(error) => {
                    self.error_message = Some(error.to_string());
                    self.error_time = None;
                }
            }
        }

        if let Ok(result) = self.details_rx.try_recv() {
            match result {
                Ok(details) => {
                    self.coin_details = Some(details);
                }

                Err(error) => {
                    self.error_message = Some(error.to_string());
                }
            }
        }

        if let Ok(result) = self.chart_rx.try_recv() {
            match result {
                Ok(chart) => {
                    self.chart_data = Some(chart);
                }

                Err(error) => {
                    self.error_message = Some(error.to_string());
                }
            }
        }

        if let Some(time) = self.error_time {
            if (Local::now() - time).num_seconds() >= 3 {
                self.error_message = None;
                self.error_time = None;
            }
        }

        // settings layout
        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .show(ctx, |ui| {
                    ui.checkbox(&mut self.settings.auto_refresh, "Auto Refresh");

                    ui.add_enabled_ui(self.settings.auto_refresh, |ui| {
                        egui::ComboBox::from_label("Interval")
                            .selected_text(format!("{} sec", self.settings.refresh_interval))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.settings.refresh_interval,
                                    5,
                                    "5 sec",
                                );
                                ui.selectable_value(
                                    &mut self.settings.refresh_interval,
                                    10,
                                    "10 sec",
                                );
                                ui.selectable_value(
                                    &mut self.settings.refresh_interval,
                                    30,
                                    "30 sec",
                                );
                                ui.selectable_value(
                                    &mut self.settings.refresh_interval,
                                    60,
                                    "1 min",
                                );
                            });
                    });
                });
        }

        if self.settings.auto_refresh
            && !self.is_refreshing
            && self.last_refresh.elapsed().as_secs() >= self.settings.refresh_interval
        {
            self.is_refreshing = true;

            let tx = self.tx.clone();
            let per_page = self.per_page;

            tokio::spawn(async move {
                let result = get_market_data(per_page).await;

                if let Err(error) = tx.send(result).await {
                    eprintln!("Failed to send result: {error}");
                }
            });

            self.last_refresh = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.current_screen {
            AppScreen::Dashboard => self.show_dashboard(ui),
            AppScreen::CoinDetails => self.show_coin_details(ui),
        });
    }
}
