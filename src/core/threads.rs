use crate::api::gw2_tp::fetch_item_names_thread;
use crate::state::threads::daemon::{daemon_thread, gc_thread, preloader_thread};

use crate::state::context::write_context;
use log::{error, info, trace};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::thread::JoinHandle;

pub(crate) static THREADS: OnceLock<Mutex<Vec<JoinHandle<()>>>> = OnceLock::new();

pub fn lock_threads() -> MutexGuard<'static, Vec<JoinHandle<()>>> {
    trace!(
        "[lock_threads] Acquiring lock (thread {:?})",
        thread::current().id()
    );
    let result = THREADS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap();
    trace!(
        "[lock_threads] Lock acquired (thread {:?})",
        thread::current().id()
    );
    result
}

pub fn unload_threads() {
    write_context().run_background_thread = false;
    let mut threads = lock_threads();
    while let Some(thread) = threads.pop() {
        info!("[unload_threads] Waiting for a thread to end..");
        match thread.join() {
            Ok(_) => info!("[unload_threads] Thread unloaded successfully"),
            Err(e) => error!("[unload_threads] Thread unloaded with error: {:?}", e),
        }
    }
}

pub fn init_threads() {
    fetch_item_names_thread();
    daemon_thread();
    gc_thread();
    preloader_thread();
}
