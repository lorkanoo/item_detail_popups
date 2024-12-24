mod preset_rule;

use crate::addon::Addon;
use function_name::named;
use log::debug;
use std::thread;
use std::time::Duration;

pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            break;
        }
        clean_finished_threads();
        thread::sleep(Duration::from_millis(500));
    }));
}

#[named]
fn clean_finished_threads() {
    Addon::threads().retain(|handle| {
        if handle.is_finished() {
            debug!("[{}] removed finished thread", function_name!());
            false
        } else {
            true
        }
    });
}
