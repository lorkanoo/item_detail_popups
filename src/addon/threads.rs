use crate::addon::{Addon, MULTITHREADED_ADDON};
use crate::api::gw2tp::fetch_item_names_thread;
use crate::thread::{gc_thread, main_background_thread, preloader_thread};

use log::{error, info, trace};
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
impl Addon {
    pub fn lock_threads() -> MutexGuard<'static, Vec<JoinHandle<()>>> {
        trace!(
            "[lock_threads] Acquiring lock (thread {:?})",
            thread::current().id()
        );
        let result = MULTITHREADED_ADDON
            .threads
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .unwrap();
        trace!(
            "[lock_threads] Lock acquired (thread {:?})",
            thread::current().id()
        );
        result
    }

    pub fn init_threads() {
        fetch_item_names_thread();
        main_background_thread();
        gc_thread();
        preloader_thread();
    }

    pub fn unload_threads() {
        Self::write_context().run_background_thread = false;
        let mut threads = Self::lock_threads();
        while let Some(thread) = threads.pop() {
            info!("[unload_threads] Waiting for a thread to end..");
            match thread.join() {
                Ok(_) => info!("[unload_threads] Thread unloaded successfully"),
                Err(e) => error!("[unload_threads] Thread unloaded with error: {:?}", e),
            }
        }
    }
}
