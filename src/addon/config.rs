use crate::{
    addon::{Addon, MULTITHREADED_ADDON},
    config::Config,
};
use log::trace;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
impl Addon {
    pub fn write_config() -> RwLockWriteGuard<'static, Config> {
        trace!(
            "[write_config] Acquiring lock (thread {:?})",
            thread::current().id()
        );
        let result = MULTITHREADED_ADDON
            .config
            .get_or_init(|| RwLock::new(Config::default()))
            .write()
            .unwrap();
        trace!(
            "[write_config] Lock acquired (thread {:?})",
            thread::current().id()
        );
        result
    }

    pub fn read_config() -> RwLockReadGuard<'static, Config> {
        trace!(
            "[read_config] Acquiring lock (thread {:?})",
            thread::current().id()
        );
        let result = MULTITHREADED_ADDON
            .config
            .get_or_init(|| RwLock::new(Config::default()))
            .read()
            .unwrap();
        trace!(
            "[read_config] Lock acquired (thread {:?})",
            thread::current().id()
        );
        result
    }
}
