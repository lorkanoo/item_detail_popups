use crate::{
    addon::{Addon, MULTITHREADED_ADDON},
    context::Context,
};
use std::sync::{Mutex, MutexGuard};

impl Addon {
    pub fn lock_context() -> MutexGuard<'static, Context> {
        MULTITHREADED_ADDON
            .context
            .get_or_init(|| Mutex::new(Context::default()))
            .lock()
            .unwrap()
    }
}
