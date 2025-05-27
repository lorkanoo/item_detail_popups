mod cloneable_clipboard;
mod links;
pub mod ui;

use crate::cache::Cache;
use crate::context::links::Links;
use crate::context::ui::UiContext;
use cloneable_clipboard::CloneableClipboard;
use nexus::imgui::sys::{self, ImFont};
use std::ffi::CStr;
use std::ptr::NonNull;
use std::slice;

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
    pub bold_font: Option<Font>,
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
            bold_font: None,
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
#[repr(transparent)]
pub struct Font(pub NonNull<ImFont>);
pub struct FontToken;

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl Font {
    pub fn as_ptr(&self) -> *mut ImFont {
        self.0.as_ptr()
    }

    pub unsafe fn get_all() -> impl Iterator<Item = Self> {
        // SAFETY: no idea
        let io = sys::igGetIO();
        let atlas = (*io).Fonts;
        let data = (*atlas).Fonts.Data;
        let len = (*atlas).Fonts.Size;

        slice::from_raw_parts(data, len as usize)
            .iter()
            .copied()
            .filter_map(NonNull::new)
            .map(Self)
    }

    pub unsafe fn name_raw<'a>(&self) -> &'a CStr {
        // SAFETY: no idea
        CStr::from_ptr(sys::ImFont_GetDebugName(self.as_ptr()))
    }

    pub fn push(&self) -> Option<FontToken> {
        self.is_valid().then(|| {
            unsafe { sys::igPushFont(self.as_ptr()) };
            FontToken
        })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { Self::get_all() }.any(|font| font == *self)
    }
}

impl Drop for FontToken {
    fn drop(&mut self) {
        unsafe { sys::igPopFont() }
    }
}
