mod client;
mod error;
mod models;
mod storage;
mod ui;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let coins = match client::get_market_data().await {
        Ok(coins) => println!("{:#?}", coins),

        Err(err) => eprintln!("Failed to fetch market data: {err}"),
    };
}
