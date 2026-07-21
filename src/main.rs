mod client;
mod error;
mod models;
mod storage;
mod ui;

use crate::ui::CryptoApp;
use dotenv::dotenv;
use eframe::NativeOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let coins = client::get_market_data()
        .await
        .expect("Failed to fetch market data");

    let options = NativeOptions::default();

    eframe::run_native(
        "Crypto Analyzer",
        options,
        Box::new(move |_cc| Ok(Box::new(CryptoApp::new(coins)))),
    )
    .expect("Failed to start GUI");
}
