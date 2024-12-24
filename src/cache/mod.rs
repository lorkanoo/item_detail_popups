use crate::addon::Addon;
use crate::config::config_dir;
use crate::context::ui::popup::Popup;
use chrono::{DateTime, Local};
use function_name::named;
use indexmap::IndexMap;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct Cache {
    pub popups: IndexMap<String, Popup>,
    pub item_names: Option<CachedData<HashMap<String, Vec<u32>>>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CachedData<T> {
    pub date: DateTime<Local>,
    pub value: T,
}


impl Cache {
    pub fn add_popup(&mut self, key: &String, popup: &mut Popup, refresh_date: bool) {
        let max_popup_cache_size = Addon::lock().config.max_popup_cache_size;
        while self.popups.len() >= max_popup_cache_size {
            if self.popups.shift_remove_index(0).is_none() {
                break;
            }
        }
        if max_popup_cache_size > 0 {
            if refresh_date {
                popup.basic_data.cached_date = Local::now();
            }
            self.popups.insert(key.clone(), popup.clone());
        }
    }
}

#[named]
pub fn try_load_item_names() -> Option<CachedData<HashMap<String, Vec<u32>>>> {
    let path = file();
    let file = File::open(&path)
        .inspect_err(|err| log::warn!("Failed to read item_names_cache: {err}"))
        .ok()?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)
        .inspect_err(|err| log::warn!("Failed to parse item_names_cache: {err}"))
        .ok()?;
    info!(
        "[{}] Loaded item_names_cache from \"{}\"",
        function_name!(),
        path.display()
    );
    Some(config)
}

#[named]
pub fn save_item_names(item_names: &Option<CachedData<HashMap<String, Vec<u32>>>>) {
    let path = file();
    match File::create(&path) {
        Ok(file) => {
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &item_names)
                .expect("failed to serialize item_names_cache");
            info!(
                "[{}] Saved popup_cache to \"{}\"",
                function_name!(),
                path.display()
            )
        }
        Err(err) => log::error!("Failed to save item_names_cache: {err}"),
    }
}

pub fn file() -> PathBuf {
    config_dir().join("item_names_cache.json")
}
