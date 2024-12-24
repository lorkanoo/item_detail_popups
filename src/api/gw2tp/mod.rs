use crate::addon::Addon;
use crate::api::get_sync;
use crate::cache::CachedData;
use chrono::{Days, Local};
use function_name::named;
use log::{debug, info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread;

const GW2TP_URL: &str = "https://api.gw2tp.com";

#[derive(Deserialize, Debug)]
struct ItemsResponse {
    items: Vec<(u32, String)>,
}

fn items_name_path() -> String {
    format!("{}/1/bulk/items-names.json", GW2TP_URL)
}

#[named]
pub fn fetch_item_names_thread() {
    Addon::threads().push(thread::spawn(|| {
        debug!("[{}] started", function_name!());
        if let Some(cache) = Addon::cache().item_names.as_ref() {
            if let Some(expiration_date) = cache.date.checked_add_days(Days::new(1)) {
                if expiration_date > Local::now() {
                    info!("[{}] cache is up to date", function_name!());
                    return;
                }
            }
        }
        match get_sync(items_name_path()) {
            Ok(response) => match response.into_json() {
                Ok(json) => {
                    let item_names: ItemsResponse = json;
                    let map_hashmap: HashMap<String, Vec<u32>> =
                        item_names
                            .items
                            .into_iter()
                            .fold(HashMap::new(), |mut map, (id, name)| {
                                map.entry(name).or_default().push(id);
                                map
                            });
                    let cache = Some(CachedData {
                        value: map_hashmap,
                        date: Local::now(),
                    });
                    Addon::cache().item_names = cache;
                }
                Err(e) => warn!("[{}] failed to fetch json: {}", function_name!(), e),
            },
            Err(_) => warn!("[{}] could not fetch item names", function_name!()),
        }
    }));
}
