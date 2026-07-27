use crate::models::Coin;
use crate::ui::{CoinFilter, SortBy, SortOrder};

use std::cmp::Ordering;
use std::collections::HashSet;

pub fn get_filtered_coins<'a>(
    coins: &'a [Coin],
    search_query: &str,
    show_filter: &CoinFilter,
    favorite_coins: &HashSet<String>,
    sort_by: &SortBy,
    sort_order: &SortOrder,
) -> Vec<&'a Coin> {
    let query = search_query.trim().to_lowercase();

    let mut coins: Vec<&Coin> = coins
        .iter()
        .filter(|coin| {
            let matches_search = query.is_empty()
                || coin.name.to_lowercase().contains(&query)
                || coin.symbol.to_lowercase().contains(&query);

            let matches_filter = match show_filter {
                CoinFilter::All => true,
                CoinFilter::Favorites => favorite_coins.contains(&coin.id),
            };

            matches_search && matches_filter
        })
        .collect();

    match (&sort_by, &sort_order) {
        (SortBy::Rank, SortOrder::Ascending) => {
            coins.sort_by(|a, b| a.market_cap_rank.cmp(&b.market_cap_rank));
        }

        (SortBy::Rank, SortOrder::Descending) => {
            coins.sort_by(|a, b| b.market_cap_rank.cmp(&a.market_cap_rank));
        }

        (SortBy::Price, SortOrder::Ascending) => {
            coins.sort_by(|a, b| {
                b.current_price
                    .partial_cmp(&a.current_price)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::Price, SortOrder::Descending) => {
            coins.sort_by(|a, b| {
                a.current_price
                    .partial_cmp(&b.current_price)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::MarketCap, SortOrder::Ascending) => {
            coins.sort_by(|a, b| {
                b.market_cap
                    .partial_cmp(&a.market_cap)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::MarketCap, SortOrder::Descending) => {
            coins.sort_by(|a, b| {
                a.market_cap
                    .partial_cmp(&b.market_cap)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::Volume, SortOrder::Ascending) => {
            coins.sort_by(|a, b| {
                b.total_volume
                    .partial_cmp(&a.total_volume)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::Volume, SortOrder::Descending) => {
            coins.sort_by(|a, b| {
                a.total_volume
                    .partial_cmp(&b.total_volume)
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::Change24h, SortOrder::Ascending) => {
            coins.sort_by(|a, b| {
                b.price_change_percentage_24h
                    .unwrap_or(0.0)
                    .partial_cmp(&a.price_change_percentage_24h.unwrap_or(0.0))
                    .unwrap_or(Ordering::Equal)
            });
        }

        (SortBy::Change24h, SortOrder::Descending) => {
            coins.sort_by(|a, b| {
                a.price_change_percentage_24h
                    .unwrap_or(0.0)
                    .partial_cmp(&b.price_change_percentage_24h.unwrap_or(0.0))
                    .unwrap_or(Ordering::Equal)
            });
        }
    }

    coins
}
