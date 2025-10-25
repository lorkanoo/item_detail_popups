use crate::api::gw2_api::fetch_prices_thread;
use crate::configuration::config::read_config;
use crate::state::cache::cache::{is_cache_expired, StoreInCache};
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Default)]
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
            match self.get(&item_id) {
                Some(price) => {
                    result.insert(item_id, price.clone());
                    if is_cache_expired(read_config().max_price_expiration_duration, price.date)
                        && !matches!(&price.caching_status, CachingStatus::InProgress)
                    {
                        if let Some(cached_price) = self.get_mut(&item_id) {
                            cached_price.caching_status = CachingStatus::InProgress;
                        }
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
            fetch_prices_thread(ids_to_cache);
        }
        Some(result)
    }
}
