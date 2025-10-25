use crate::state::cache::caching_status::CachingStatus;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CachedData<T> {
    pub caching_status: CachingStatus,
    pub(crate) date: DateTime<Local>,
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
