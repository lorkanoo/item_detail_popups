use nexus::imgui::sys::{self, ImFont};
use std::ffi::CStr;
use std::ptr::NonNull;
use std::slice;

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
