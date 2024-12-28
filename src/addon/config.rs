use crate::{
    addon::{Addon, MULTITHREADED_ADDON},
    config::Config,
};
use std::sync::{Mutex, MutexGuard};

impl Addon {
    pub fn lock_config() -> MutexGuard<'static, Config> {
        MULTITHREADED_ADDON
            .config
            .get_or_init(|| Mutex::new(Config::default()))
            .lock()
            .unwrap()
    }
}
