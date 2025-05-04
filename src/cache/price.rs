use crate::addon::Addon;
use crate::api::gw2_api::fetch_prices_thread;
use crate::cache::{CachedData, CachingStatus};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{is_cache_expired, Cacheable};
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Price {
    pub highest_buy: u32,
    pub lowest_sell: u32,
}

pub type PriceCache = HashMap<u32, CachedData<Price>>;

impl<'a> Cacheable<'a, PriceCache, PriceCache, Vec<u32>> for PriceCache {
    fn retrieve(&'a mut self, key: Vec<u32>) -> Option<PriceCache> {
        let mut prices_to_cache = HashMap::new();
        let mut result = HashMap::new();
        for item_id in key {
            match self.get(&item_id) {
                Some(price) => {
                    result.insert(item_id, price.clone());
                    if is_cache_expired(
                        Addon::read_config().max_price_expiration_duration,
                        price.date,
                    ) && !matches!(&price.caching_status, CachingStatus::InProgress)
                    {
                        let new: CachedData<Price> =
                            CachedData::new_with_value(Local::now(), Price::default());
                        prices_to_cache.insert(item_id, new.clone());
                    }
                }
                None => {
                    let price_to_cache = CachedData::new_with_value(Local::now(), Price::default());
                    prices_to_cache.insert(item_id, price_to_cache.clone());
                    result.insert(item_id, price_to_cache);
                }
            }
        }
        if !prices_to_cache.is_empty() {
            fetch_prices_thread(prices_to_cache);
        }
        Some(result)
    }
}
