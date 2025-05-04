use crate::{
    addon::{Addon, MULTITHREADED_ADDON},
    context::Context,
};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use log::debug;
use std::thread;

impl Addon {
    pub fn write_context() -> RwLockWriteGuard<'static, Context> {
        debug!("[write_context] Acquiring lock (thread {:?})", thread::current().id());
        let result = MULTITHREADED_ADDON
            .context
            .get_or_init(|| RwLock::new(Context::default()))
            .write()
            .unwrap();
        debug!("[write_context] Lock acquired (thread {:?})", thread::current().id());
        result
        
    }
    
    pub fn read_context() -> RwLockReadGuard<'static, Context> {
        debug!("[read_context] Acquiring lock (thread {:?})", thread::current().id());
        let result = MULTITHREADED_ADDON
            .context
            .get_or_init(|| RwLock::new(Context::default()))
            .read()
            .unwrap();
        debug!("[read_context] Lock acquired (thread {:?})", thread::current().id());
        result
    }

}
