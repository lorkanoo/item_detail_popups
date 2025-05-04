use crate::addon::Addon;
use crate::api::get_sync;
use crate::cache::price::Price;
use crate::cache::CachedData;
use crate::cache::CachingStatus::Cached;
use chrono::Local;

use log::{debug, error, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread;

const GW2_API_URL: &str = "https://api.guildwars2.com/v2";

#[derive(Deserialize, Debug)]
pub struct PriceData {
    pub id: u32,
    pub buys: PriceDetails,
    pub sells: PriceDetails,
}

#[derive(Deserialize, Debug)]
pub struct PriceDetails {
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
    Addon::lock_threads().push(thread::spawn(move || {
        let item_ids: Vec<u32> = prices_to_cache.keys().copied().collect();
        debug!("[fetch_prices_thread] started for {} items", item_ids.len());
        match get_sync(prices_path(&item_ids)) {
            Ok(response) => match response.into_json::<Vec<PriceData>>() {
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

                            Addon::write_context()
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
