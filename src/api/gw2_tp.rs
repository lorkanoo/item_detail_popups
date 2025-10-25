use crate::core::http_client::get_sync;
use chrono::Local;

use crate::core::threads::lock_threads;
use crate::state::cache::cache::is_cache_expired;
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus::Cached;
use crate::state::context::{read_context, write_context};
use log::{debug, error};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

const GW2TP_URL: &str = "https://api.gw2tp.com";
const ITEM_NAMES_CACHE_EXPIRATION: Duration = Duration::from_secs(60 * 60 * 24);

#[derive(Deserialize, Debug)]
struct ItemsResponse {
    items: Vec<(u32, String)>,
}

fn items_name_path() -> String {
    format!("{}/1/bulk/items-names.json", GW2TP_URL)
}

pub fn fetch_item_names_thread() {
    lock_threads().push(thread::spawn(|| {
        debug!("[fetch_item_names_thread] started");
        let date = read_context().cache.item_names.date();
        if !is_cache_expired(ITEM_NAMES_CACHE_EXPIRATION, date) {
            debug!("[fetch_item_names_thread] cache is up to date");
            return;
        }

        let response = get_sync(items_name_path());
        if let Err(e) = response {
            error!("[get_sync] could not fetch item names: {}", e);
            return;
        }
        let response = response.unwrap();
        let json = response.into_json();
        if let Err(e) = json {
            error!("[get_sync] failed to fetch json: {}", e);
            return;
        }

        let item_names: ItemsResponse = json.unwrap();
        let ids_by_item_name_map: HashMap<String, Vec<u32>> =
            item_names
                .items
                .into_iter()
                .fold(HashMap::new(), |mut map, (id, name)| {
                    map.entry(name).or_default().push(id);
                    map
                });

        write_context().cache.item_names =
            CachedData::new_with_value(Local::now(), ids_by_item_name_map)
                .with_caching_status(Cached);
    }));
}
