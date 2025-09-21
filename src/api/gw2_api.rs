use crate::core::http_client::get_sync;
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus::Cached;
use crate::state::cache::price::Price;
use chrono::Local;

use log::{debug, error};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread;
use crate::state::context::write_context;
use crate::core::threads::lock_threads;

const GW2_API_URL: &str = "https://api.guildwars2.com/v2";

#[derive(Deserialize, Debug)]
pub struct PriceApiResponse {
    pub id: u32,
    pub buys: PriceResponse,
    pub sells: PriceResponse,
}

#[derive(Deserialize, Debug)]
pub struct PriceResponse {
    pub unit_price: u32,
}

fn prices_path(ids: &[u32]) -> String {
    format!(
        "{}/commerce/prices?ids={}",
        GW2_API_URL,
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub fn fetch_prices_thread(prices_to_cache: HashMap<u32, CachedData<Price>>) {
    lock_threads().push(thread::spawn(move || {
        let item_ids: Vec<u32> = prices_to_cache.keys().copied().collect();
        debug!("[fetch_prices_thread] started for {} items", item_ids.len());
        match get_sync(prices_path(&item_ids)) {
            Ok(response) => match response.into_json::<Vec<PriceApiResponse>>() {
                Ok(prices) => {
                    for price_data in prices {
                        if prices_to_cache.contains_key(&price_data.id) {
                            let new_price = Price {
                                highest_buy: price_data.buys.unit_price,
                                lowest_sell: price_data.sells.unit_price,
                            };
                            let new_cached_price =
                                CachedData::new_with_value(Local::now(), new_price)
                                    .with_caching_status(Cached);

                            write_context()
                                .cache
                                .prices
                                .insert(price_data.id, new_cached_price);
                        }
                    }
                }
                Err(e) => error!("[get_sync] failed to parse prices json: {}", e),
            },
            Err(e) => error!("[get_sync] failed to fetch prices: {}", e),
        }
    }));
}
