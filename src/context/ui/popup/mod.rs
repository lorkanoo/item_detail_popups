use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;
use popup_data::PopupData;
pub mod dimensions;
pub mod popup_data;
pub mod style;
pub mod tag_params;
pub mod token;

static POPUP_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(|| {
    AtomicU64::new(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
});

#[derive(Clone, Debug)]
pub struct Popup {
    pub id: u64,
    pub opened: bool,
    pub pinned: bool,
    pub collapsed: bool,
    pub title_dragging: bool,
    pub pos: Option<[f32; 2]>,
    pub width: Option<f32>,
    pub data: PopupData,
}

impl Popup {
    pub fn new(data: PopupData) -> Self {
        let id = POPUP_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            opened: false,
            pinned: false,
            title_dragging: false,
            pos: None,
            width: None,
            data,
            collapsed: false,
        }
    }
}
