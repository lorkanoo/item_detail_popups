mod cloneable_clipboard;
mod links;
pub mod ui;

use crate::cache::Cache;
use crate::context::links::Links;
use crate::context::ui::UiContext;
use cloneable_clipboard::CloneableClipboard;

#[derive(Clone)]
pub struct Context {
    pub links: Links,
    pub run_background_thread: bool,
    pub ui: UiContext,
    pub clipboard: CloneableClipboard,
    pub last_clipboard_text: Option<String>,
    pub cache: Cache,
    pub search_text: String,
    pub should_open_search: bool,
    pub search_opened: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            links: Links::default(),
            run_background_thread: true,
            ui: UiContext::default(),
            clipboard: CloneableClipboard::default(),
            last_clipboard_text: None,
            cache: Cache::default(),
            search_text: "".to_string(),
            should_open_search: false,
            search_opened: false,
        }
    }
}
