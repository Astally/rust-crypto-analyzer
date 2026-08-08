use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub market_cap_rank: u32,
    pub total_volume: f64,
    pub price_change_percentage_24h: Option<f64>,
    pub last_updated: String,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub ath: f64,
    pub atl: f64,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub homepage: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CoinDetails {
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Chart {
    pub prices: Vec<(f64, f64)>,
}
