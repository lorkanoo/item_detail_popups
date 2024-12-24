use crate::addon::{Addon, MULTITHREADED_ADDON};
use crate::cache::Cache;
use std::sync::{Mutex, MutexGuard};

impl Addon {
    pub fn cache() -> MutexGuard<'static, Cache> {
        MULTITHREADED_ADDON
            .cache
            .get_or_init(|| Mutex::new(Cache::default()))
            .lock()
            .unwrap()
    }
}
