mod client;
mod error;
mod models;
mod storage;
mod ui;

use crate::ui::CryptoApp;
use dotenv::dotenv;
use eframe::NativeOptions;

const DEFAULT_PER_PAGE: u32 = 10;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let per_page = DEFAULT_PER_PAGE;

    let coins = client::get_market_data(per_page)
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
