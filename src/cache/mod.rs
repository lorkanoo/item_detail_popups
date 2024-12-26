mod item_name;
mod popup;
pub mod price;

use crate::cache::price::Price;
use crate::context::ui::popup::Popup;
use chrono::{DateTime, Local};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Cache {
    pub popups: IndexMap<String, Popup>,
    pub item_names: CachedData<HashMap<String, Vec<u32>>>,
    pub prices: HashMap<u32, CachedData<Price>>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CachedData<T> {
    pub caching_status: CachingStatus,
    date: DateTime<Local>,
    value: T,
}

impl<T> CachedData<T> {
    pub fn new(date: DateTime<Local>, value: T) -> Self {
        Self {
            caching_status: Default::default(),
            date,
            value,
        }
    }
    pub fn value(&self) -> Option<&T> {
        if matches!(self.caching_status, CachingStatus::Cached) {
            return Some(&self.value);
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

#[derive(Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum CachingStatus {
    #[default]
    NotCached,
    InProgress,
    Cached,
}

