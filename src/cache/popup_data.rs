use crate::addon::Addon;
use crate::config::config_dir;
use crate::context::ui::popup::PopupData;

use super::{is_cache_expired, Cacheable, Persistent};
use indexmap::IndexMap;
use log::{debug, info, warn};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub type PopupDataCache = IndexMap<String, PopupData>;

impl Persistent for PopupDataCache {
    fn load(&mut self) {
        let path = PopupDataCache::file_path();
        let file_opt = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read popup_cache: {err}"))
            .ok();
        if file_opt.is_none() {
            warn!(
                "[load] Failed to load item names cache from \"{}\"",
                path.display()
            );
            return;
        }
        let file = file_opt.unwrap();
        let reader = BufReader::new(file);
        let config_opt = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse popup_cache: {err}"))
            .ok();
        if let Some(config) = config_opt {
            *self = config;
        }
        info!(
            "[load_popups] Loaded popup_cache from \"{}\"",
            path.display()
        );
    }

    fn file_path() -> PathBuf {
        config_dir().join("popup_data_cache.json")
    }

    fn save(&self) {
        let path = PopupDataCache::file_path();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self)
                    .expect("failed to serialize popup_data_cache");
                info!(
                    "[save_popup_data_map] Saved popup data cache to \"{}\"",
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save popup data cache: {err}"),
        }
    }
}

impl<'a> Cacheable<'a, PopupDataCache, PopupData, &'a String, &'a mut PopupData>
    for PopupDataCache
{
    fn retrieve(&'a mut self, key: &'a String) -> Option<PopupData> {
        debug!(
            "[retrieve] Attempting to retrieve popup data for href: {}",
            key
        );

        let cached_data = self.swap_remove_entry(key);
        if let Some((_, mut cached_data)) = cached_data {
            let cache_expiration = Addon::lock_config().max_popup_data_cache_expiration_duration;
            if !is_cache_expired(cache_expiration, cached_data.cached_date) {
                PopupDataCache::store(self, key, &mut cached_data);
                return Some(cached_data);
            }
        }
        None
    }

    fn store(&'a mut self, key: &'a String, value: &'a mut PopupData) {
        let max_popup_data_cache_size = Addon::lock_config().max_popup_data_cache_elements;
        while self.len() >= max_popup_data_cache_size {
            if self.shift_remove_index(0).is_none() {
                break;
            }
        }
        if max_popup_data_cache_size > 0 {
            self.insert(key.to_owned(), value.clone());
        }
    }
}
