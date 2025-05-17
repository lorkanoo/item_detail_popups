use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use popup_data::PopupData;
pub mod popup_data;
pub mod style;
pub mod tag_params;
pub mod token;
pub mod dimensions;

static POPUP_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(|| {
    AtomicU64::new(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Popup {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub id: u64,
    pub opened: bool,
    pub pinned: bool,
    pub pos: Option<[f32; 2]>,
    pub data: PopupData,
}

impl Popup {
    pub fn new(data: PopupData) -> Self {
        let id = POPUP_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            opened: false,
            pinned: false,
            pos: None,
            data,
        }
    }
}
