use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use chrono::Local;
use log::debug;
use crate::api::gw2_tp::get_item_names;
use crate::core::threads::lock_threads;
use crate::state::cache::cache::is_cache_expired;
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus::Cached;
use crate::state::context::{read_context, write_context};

const ITEM_NAMES_CACHE_EXPIRATION: Duration = Duration::from_secs(60 * 60 * 24);

pub fn cache_item_names_thread() {
    lock_threads().push(thread::spawn(|| {
        debug!("[cache_item_names_thread] started");
        let date = read_context().cache.item_names.date();
        if !is_cache_expired(ITEM_NAMES_CACHE_EXPIRATION, date) {
            debug!("[fetch_item_names_thread] cache is up to date");
            return;
        }

        match get_item_names() {
            Ok(items_response) => {
                let ids_grouped_by_item_name: HashMap<String, Vec<u32>> =
                    items_response
                        .items
                        .into_iter()
                        .fold(HashMap::new(), |mut map, (id, name)| {
                            map.entry(name).or_default().push(id);
                            map
                        });

                write_context().cache.item_names =
                    CachedData::new_with_value(Local::now(), ids_grouped_by_item_name)
                        .with_caching_status(Cached);
            }
            Err(api_error) => api_error.log()
        }
    }));
}