mod cloneable_clipboard;
mod links;
pub mod ui;

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
}

impl Default for Context {
    fn default() -> Self {
        Self {
            links: Links::default(),
            run_background_thread: true,
            ui: UiContext::default(),
            clipboard: CloneableClipboard::default(),
            last_clipboard_text: None,
        }
    }
}
