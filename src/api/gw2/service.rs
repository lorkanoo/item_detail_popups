use crate::core::http_client::get_sync;
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus::{Cached, Failed};
use crate::state::cache::price::Price;
use chrono::Local;

use crate::api::gw2::response::PriceApiResponse;
use crate::core::threads::lock_threads;
use crate::state::context::write_context;
use log::{debug, error, warn};
use std::thread;

const GW2_API_URL: &str = "https://api.guildwars2.com/v2";

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

pub fn cache_prices_thread(ids_to_cache: Vec<u32>) {
    lock_threads().push(thread::spawn(move || {
        debug!(
            "[fetch_prices_thread] started for {} items",
            ids_to_cache.len()
        );

        match get_sync(prices_path(&ids_to_cache)) {
            Ok(response) => match response.into_json::<Vec<PriceApiResponse>>() {
                Ok(prices) => {
                    cache_prices(&ids_to_cache, prices);
                }
                Err(e) => error!("[get_sync] failed to parse prices json: {}", e),
            },
            Err(e) => {
                mark_price_caching_as_failed(ids_to_cache);
                warn!("[get_sync] failed to fetch prices: {}", e)
            }
        }
    }));
}

fn mark_price_caching_as_failed(ids_to_cache: Vec<u32>) {
    for id in ids_to_cache {
        write_context().cache.prices.insert(
            id,
            CachedData::new(Local::now()).with_caching_status(Failed),
        );
    }
}

fn cache_prices(ids_to_cache: &Vec<u32>, prices: Vec<PriceApiResponse>) {
    for price_data in prices {
        if !ids_to_cache.contains(&price_data.id) {
            continue;
        }
        cache_price(price_data);
    }
}

fn cache_price(price_data: PriceApiResponse) {
    let new_price = Price {
        highest_buy: price_data.buys.unit_price,
        lowest_sell: price_data.sells.unit_price,
    };
    let new_cached_price =
        CachedData::new_with_value(Local::now(), new_price).with_caching_status(Cached);

    write_context()
        .cache
        .prices
        .insert(price_data.id, new_cached_price);
}
