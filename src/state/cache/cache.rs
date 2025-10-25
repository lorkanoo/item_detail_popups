use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

use crate::state::cache::cached_data::CachedData;
use crate::state::cache::item_name::ItemNamesCache;
use crate::state::cache::price::PriceCache;
use crate::state::cache::texture::TextureCache;
use crate::state::popup::popup_data::PopupDataCache;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Cache {
    pub popup_data_map: PopupDataCache,
    pub item_names: CachedData<ItemNamesCache>,
    pub prices: PriceCache,
    #[serde(skip_serializing, skip_deserializing)]
    pub textures: TextureCache,
}

impl Cache {
    pub fn evict(&mut self) {
        self.popup_data_map.clear();
        self.prices.clear();
        self.textures.clear();
    }
}

pub fn is_cache_expired(cache_expiration: Duration, cached_on_date: DateTime<Local>) -> bool {
    cached_on_date + cache_expiration <= Local::now()
}

pub trait StoreInCache<'a, T, R = T, K = (), V = T> {
    fn retrieve(&'a mut self, key: K) -> Option<R>;
    fn store(&'a mut self, _key: K, _value: V) {
        panic!("not implemented");
    }
}

pub trait Persist {
    fn load(&mut self);
    fn save(&self);
    fn file_path() -> PathBuf;
}
