use crate::addon::Addon;
use crate::api::gw2_api::fetch_prices_thread;
use crate::cache::{Cache, CachedData, CachingStatus};
use chrono::{Local, TimeDelta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Price {
    pub highest_buy: u32,
    pub lowest_sell: u32,
}

impl Cache {
    pub fn prices(
        item_ids: Vec<u32>,
        price_expiration_sec: i64,
    ) -> HashMap<u32, CachedData<Price>> {
        let mut prices_to_cache = HashMap::new();
        let mut result = HashMap::new();
        for item_id in item_ids {
            match Addon::cache().prices.get(&item_id) {
                Some(price) => {
                    result.insert(item_id, price.clone());
                    if let Some(expiration_date) = price
                        .date
                        .checked_add_signed(TimeDelta::seconds(price_expiration_sec))
                    {
                        if Local::now() > expiration_date
                            && !matches!(&price.caching_status, CachingStatus::InProgress)
                        {
                            let new = CachedData::new(Local::now(), Price::default())
                                .with_caching_status(CachingStatus::InProgress);
                            prices_to_cache.insert(item_id, new.clone());
                        }
                    }
                }
                None => {
                    let new = CachedData::new(Local::now(), Price::default())
                        .with_caching_status(CachingStatus::InProgress);
                    prices_to_cache.insert(item_id, new.clone());
                    result.insert(item_id, new);
                }
            }
        }
        if !prices_to_cache.is_empty() {
            fetch_prices_thread(prices_to_cache);
        }

        result
    }
}
