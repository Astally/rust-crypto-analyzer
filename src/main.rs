mod client;
mod models;
mod storage;
mod ui;

use crate::ui::CryptoApp;
use anyhow::Result;
use dotenv::dotenv;
use eframe::NativeOptions;
use tokio::sync::mpsc::channel;

const DEFAULT_PER_PAGE: u32 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let (tx, rx) = channel(1);

    let coins = client::get_market_data(DEFAULT_PER_PAGE).await?;

    let options = NativeOptions::default();

    if let Err(error) = eframe::run_native(
        "Crypto Analyzer",
        options,
        Box::new(move |_cc| Ok(Box::new(CryptoApp::new(coins, tx, rx)))),
    ) {
        eprintln!("Failed to start GUI: {error}");
    }

    Ok(())
}
