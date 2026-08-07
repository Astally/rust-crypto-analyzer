mod client;
mod dashboard;
mod details;
mod filter;
mod format;
mod models;
mod storage;
mod ui;

use crate::ui::CryptoApp;
use anyhow::Result;
use dotenv::dotenv;
use eframe::NativeOptions;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let (tx, rx) = channel(1);

    let (details_tx, details_rx) = channel(1);

    let coins = client::get_market_data(10).await?;

    let options = NativeOptions::default();

    if let Err(error) = eframe::run_native(
        "Crypto Analyzer",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(CryptoApp::new(
                coins, tx, rx, details_tx, details_rx,
            )))
        }),
    ) {
        eprintln!("Failed to start GUI: {error}");
    }

    Ok(())
}
