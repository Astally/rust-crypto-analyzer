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

    let options = NativeOptions::default();

    eframe::run_native(
        "Crypto Analyzer",
        options,
        Box::new(|_cc| Ok(Box::new(CryptoApp::new()))),
    )
    .expect("Failed to start GUI");
}
