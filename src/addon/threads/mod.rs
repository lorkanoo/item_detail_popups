use crate::addon::{Addon, MULTITHREADED_ADDON};
use function_name::named;
use log::info;
use std::sync::{Mutex, MutexGuard};
use std::thread::JoinHandle;

impl Addon {
    pub fn threads() -> MutexGuard<'static, Vec<JoinHandle<()>>> {
        MULTITHREADED_ADDON
            .threads
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .unwrap()
    }

    #[named]
    pub fn unload_threads() {
        Self::lock().context.run_background_thread = false;
        let mut threads = Self::threads();
        while let Some(thread) = threads.pop() {
            info!("[{}] Waiting for a thread to end..", function_name!());
            match thread.join() {
                Ok(_) => info!("[{}] Thread unloaded successfully", function_name!()),
                Err(_) => log::error!("[{}] Thread unloaded with error", function_name!()),
            }
        }
    }
}
