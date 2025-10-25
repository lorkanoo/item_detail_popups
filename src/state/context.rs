use crate::state::cache::cache::Cache;
use crate::state::cache::cache::Persist;
use crate::state::clipboard::CustomClipboard;
use crate::state::font::Font;
use crate::state::links::Links;
use crate::state::ui_context::UiContext;
use log::trace;
use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;

pub(crate) static CONTEXT: OnceLock<RwLock<Context>> = OnceLock::new();

#[derive(Clone)]
pub struct Context {
    pub links: Links,
    pub run_background_thread: bool,
    pub ui: UiContext,
    pub clipboard: CustomClipboard,
    pub last_clipboard_text: Option<String>,
    pub cache: Cache,
    pub search_popup_input: String,
    pub should_open_search: bool,
    pub search_opened: bool,
    pub bold_font: Option<Font>,
    pub tab_to_blacklist_input: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            links: Links::default(),
            run_background_thread: true,
            ui: UiContext::default(),
            clipboard: CustomClipboard::default(),
            last_clipboard_text: None,
            cache: Cache::default(),
            search_popup_input: "".to_string(),
            should_open_search: false,
            search_opened: false,
            bold_font: None,
            tab_to_blacklist_input: "".to_string(),
        }
    }
}

pub fn write_context() -> RwLockWriteGuard<'static, Context> {
    trace!(
        "[write_context] Acquiring lock (thread {:?})",
        thread::current().id()
    );
    let result = CONTEXT
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
    let result = CONTEXT
        .get_or_init(|| RwLock::new(Context::default()))
        .read()
        .unwrap();
    trace!(
        "[read_context] Lock acquired (thread {:?})",
        thread::current().id()
    );
    result
}

pub(crate) fn save_cache() {
    read_context().cache.item_names.save();
    read_context().cache.popup_data_map.save();
}
