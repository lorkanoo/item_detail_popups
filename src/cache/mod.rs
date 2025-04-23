pub mod item_name;
pub mod popup_data;
pub mod price;
pub mod texture;

use chrono::{DateTime, Local};
use item_name::ItemNamesCache;
use popup_data::PopupDataCache;
use texture::TextureCache;
use price::PriceCache;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Cache {
    pub popup_data_map: PopupDataCache,
    pub item_names: CachedData<ItemNamesCache>,
    pub prices: PriceCache,
    #[serde(skip_serializing, skip_deserializing)]
    pub textures: TextureCache,
}

pub trait Cacheable<'a, T, R = T, K = (), V = T> {
    fn retrieve(&'a mut self, key: K) -> Option<R>;
    fn store(&'a mut self, _key: K, _value: V) {
        panic!("not implemented");
    }
}

pub trait Persistent {
    fn load(&mut self);
    fn save(&self);
    fn file_path() -> PathBuf;
}

impl Cache {
    pub fn evict(&mut self) {
        self.popup_data_map.clear();
        self.prices.clear();
        self.textures.clear();
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CachedData<T> {
    pub caching_status: CachingStatus,
    date: DateTime<Local>,
    value: Option<T>,
}

impl<T> CachedData<T> {
    pub fn new(date: DateTime<Local>) -> Self {
        Self {
            caching_status: CachingStatus::InProgress,
            date,
            value: None,
        }
    }
    pub fn new_with_value(date: DateTime<Local>, value: T) -> Self {
        Self {
            caching_status: CachingStatus::InProgress,
            date,
            value: Some(value),
        }
    }

    pub fn value(&self) -> Option<&T> {
        if matches!(self.caching_status, CachingStatus::Cached) {
            return self.value.as_ref();
        }
        None
    }

    pub fn with_caching_status(mut self, status: CachingStatus) -> Self {
        self.caching_status = status;
        self
    }

    pub fn date(&self) -> DateTime<Local> {
        self.date
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum CachingStatus {
    #[default]
    InProgress,
    Cached,
}

pub fn is_cache_expired(cache_expiration: Duration, cached_on_date: DateTime<Local>) -> bool {
    cached_on_date + cache_expiration <= Local::now()
}
