use crate::models::{Coin, CoinDetails};
use anyhow::Result;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

const BASE_URL: &str = "https://api.coingecko.com/api/v3/coins/markets";

const DEFAULT_CURRENCY: &str = "usd";

const DEFAULT_ORDER: &str = "market_cap_desc";

const DEFAULT_PAGE: u32 = 1;

pub async fn get_market_data(per_page: u32) -> Result<Vec<Coin>> {
    let api_key = env::var("API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert("x-cg-demo-api-key", HeaderValue::from_str(&api_key)?);

    let client = Client::new();

    let response = client
        .get(format!(
            "{BASE_URL}?vs_currency={DEFAULT_CURRENCY}&order={DEFAULT_ORDER}&per_page={per_page}&page={DEFAULT_PAGE}"
        ))
        .headers(headers)
        .send()
        .await?;

    let safe_response = response.error_for_status()?;

    let json_response: Vec<Coin> = safe_response.json().await?;

    Ok(json_response)
}

pub async fn get_link_data(coin_id: &str) -> Result<CoinDetails> {
    let api_key = env::var("API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert("x-cg-demo-api-key", HeaderValue::from_str(&api_key)?);

    let client = Client::new();

    let response = client
        .get(format!("https://api.coingecko.com/api/v3/coins/{coin_id}"))
        .headers(headers)
        .send()
        .await?;

    let safe_response = response.error_for_status()?;

    let json_response: CoinDetails = safe_response.json().await?;

    Ok(json_response)
}
