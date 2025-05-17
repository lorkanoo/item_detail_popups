use crate::{
    addon::{Addon, MULTITHREADED_ADDON},
    context::Context,
};
use log::trace;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;

impl Addon {
    pub fn write_context() -> RwLockWriteGuard<'static, Context> {
        trace!(
            "[write_context] Acquiring lock (thread {:?})",
            thread::current().id()
        );
        let result = MULTITHREADED_ADDON
            .context
            .get_or_init(|| RwLock::new(Context::default()))
            .write()
            .unwrap();
        trace!(
            "[write_context] Lock acquired (thread {:?})",
            thread::current().id()
        );
        result
    }

    pub fn read_context() -> RwLockReadGuard<'static, Context> {
        trace!(
            "[read_context] Acquiring lock (thread {:?})",
            thread::current().id()
        );
        let result = MULTITHREADED_ADDON
            .context
            .get_or_init(|| RwLock::new(Context::default()))
            .read()
            .unwrap();
        trace!(
            "[read_context] Lock acquired (thread {:?})",
            thread::current().id()
        );
        result
    }
}
