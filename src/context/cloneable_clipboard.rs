use arboard::{Clipboard, Error};

pub struct CloneableClipboard {
    pub clipboard: Clipboard,
}

impl Default for CloneableClipboard {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl Clone for CloneableClipboard {
    fn clone(&self) -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl CloneableClipboard {
    pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
        self.clipboard.set_text(text)
    }
    pub fn get_text(&mut self) -> Result<String, Error> {
        self.clipboard.get_text()
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.clipboard.clear()
    }
}
