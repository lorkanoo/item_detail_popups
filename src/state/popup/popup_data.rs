use std::collections::BTreeMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::token::Token;
use crate::configuration::config::config_dir;

use crate::configuration::config::read_config;
use crate::state::cache::cache::{is_cache_expired, Persist, StoreInCache};
use indexmap::IndexMap;
use log::{debug, info, trace, warn};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub type PopupDataCache = IndexMap<String, PopupData>;
pub type SectionName = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopupData {
    pub item_ids: Option<Vec<u32>>,
    pub item_icon: Option<Token>,
    pub title: String,
    pub description: Vec<Token>,
    pub sections: IndexMap<SectionName, Vec<Token>>,
    pub images: Vec<Token>,
    // tag href, tag name
    pub tags: BTreeMap<String, String>,
    pub cached_date: DateTime<Local>,
    pub href: String,
    pub redirection_href: Option<String>,
}

impl PopupData {
    pub fn is_not_empty(&self) -> bool {
        !self.description.is_empty()
            || self.sections.iter().any(|(_, tokens)| !tokens.is_empty())
            || !self.images.is_empty()
            || self.item_ids.is_some()
    }
}

impl Default for PopupData {
    fn default() -> Self {
        Self {
            item_ids: None,
            item_icon: None,
            title: "".to_string(),
            description: vec![],
            sections: Default::default(),
            images: vec![],
            tags: BTreeMap::new(),
            cached_date: Local::now(),
            href: "".to_string(),
            redirection_href: None,
        }
    }
}

impl Persist for PopupDataCache {
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

    fn save(&self) {
        let path = PopupDataCache::file_path();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self).expect("failed to serialize popup_data");
                info!(
                    "[save_popup_data_map] Saved popup data cache to \"{}\"",
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save popup data cache: {err}"),
        }
    }

    fn file_path() -> PathBuf {
        config_dir().join("popup_data.json")
    }
}

impl<'a> StoreInCache<'a, PopupDataCache, PopupData, &'a String, &'a mut PopupData>
    for PopupDataCache
{
    fn retrieve(&'a mut self, key: &'a String) -> Option<PopupData> {
        debug!(
            "[retrieve] Attempting to retrieve popup data for href: {}",
            key
        );

        let cached_data = self.swap_remove_entry(key);
        if let Some((_, mut cached_data)) = cached_data {
            let cache_expiration = read_config().max_popup_data_expiration_duration;
            if !is_cache_expired(cache_expiration, cached_data.cached_date) {
                PopupDataCache::store(self, key, &mut cached_data);
                return Some(cached_data);
            }
        }
        None
    }

    fn store(&'a mut self, key: &'a String, value: &'a mut PopupData) {
        let max_popup_data_size = read_config().max_popup_data_elements;
        while self.len() >= max_popup_data_size {
            trace!("[store] loop");
            if self.shift_remove_index(0).is_none() {
                break;
            }
        }
        if max_popup_data_size > 0 {
            self.insert(key.to_owned(), value.clone());
        }
    }
}
