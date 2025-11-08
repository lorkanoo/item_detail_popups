use crate::api::gw2::price::{get_prices, PriceApiResponse};
use crate::configuration::config::read_config;
use crate::state::cache::cache::{is_cache_expired, StoreInCache};
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;
use log::debug;
use crate::core::threads::lock_threads;
use crate::state::cache::caching_status::CachingStatus::{Cached, Failed};
use crate::state::context::write_context;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Price {
    pub highest_buy: u32,
    pub lowest_sell: u32,
}

pub type PriceCache = HashMap<u32, CachedData<Price>>;

impl<'a> StoreInCache<'a, PriceCache, PriceCache, Vec<u32>> for PriceCache {
    fn retrieve(&'a mut self, key: Vec<u32>) -> Option<PriceCache> {
        let mut ids_to_cache = vec![];
        let mut result = HashMap::new();
        for item_id in key {
            match self.get_mut(&item_id) {
                Some(price) => {
                    result.insert(item_id, price.clone());
                    if is_cache_expired(read_config().max_price_expiration_duration, price.date)
                        && !matches!(&price.caching_status, CachingStatus::Refreshing)
                    {
                        price.caching_status = CachingStatus::Refreshing;
                        ids_to_cache.push(item_id);
                    }
                }
                None => {
                    let price_to_cache = CachedData::new_with_value(Local::now(), Price::default());
                    ids_to_cache.push(item_id);
                    self.insert(item_id, price_to_cache.clone());
                    result.insert(item_id, price_to_cache);
                }
            }
        }
        if !ids_to_cache.is_empty() {
            cache_prices_thread(ids_to_cache);
        }
        Some(result)
    }
}

pub fn cache_prices_thread(item_ids: Vec<u32>) {
    lock_threads().push(thread::spawn(move || {
        debug!(
            "[cache_prices_thread] started for {} items",
            item_ids.len()
        );

        match get_prices(&item_ids) {
            Ok(prices) => cache_prices(&item_ids, prices),
            Err(api_error) => {
                api_error.log();
                mark_price_caching_as_failed(item_ids);
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
