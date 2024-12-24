mod links;
pub mod ui;

use crate::addon::Addon;
use crate::context::links::Links;
use crate::context::ui::UiContext;
use arboard::{Clipboard, Error};

#[derive(Clone)]
pub struct Context {
    pub links: Links,
    pub run_background_thread: bool,
    pub ui: UiContext,
    pub clipboard: CustomClipboard,
}

pub struct CustomClipboard {
    pub clipboard: Clipboard,
}

impl Clone for CustomClipboard {
    fn clone(&self) -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl CustomClipboard {
    pub fn get_text(&mut self) -> Result<String, Error> {
        self.clipboard.get_text()
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.clipboard.clear()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            links: Default::default(),
            run_background_thread: true,
            ui: UiContext::default(),
            clipboard: CustomClipboard {
                clipboard: Clipboard::new().unwrap(),
            },
        }
    }
}
impl Context {
    pub fn valid(&self) -> bool {
        true
    }
}

pub fn init_context() {
    if Addon::lock().config.valid() {}
}
