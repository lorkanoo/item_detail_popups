use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::render::util::ui::UiElement;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

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
    pub basic_data: BasicData,
}

impl Popup {
    pub fn new(basic_data: BasicData) -> Self {
        let id = POPUP_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            opened: false,
            basic_data,
        }
    }
    pub fn assign_id_and_clone(&self) -> Popup {
        let id = POPUP_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut clone = self.clone();
        clone.id = id;
        clone
    }
}

impl UiElement for Popup {
    fn rename(&mut self, new_name: String) {
        self.basic_data.title = new_name;
    }

    fn name(&self) -> &String {
        &self.basic_data.title
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicData {
    pub item_ids: Option<Vec<u32>>,
    pub title: String,
    pub description: Vec<Token>,
    pub notes: Vec<Token>,
    pub acquisition: Vec<Token>,
    // tag href, tag name
    pub tags: BTreeMap<String, String>,
    pub pinned: bool,
    pub pos: Option<[f32; 2]>,
    pub cached_date: DateTime<Local>,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagParams {
    pub href: String,
    pub text: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Token {
    Text(String, Style),
    Tag(TagParams),
    Spacing,
    ListElement,
    Indent(i32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Style {
    Normal,
    Highlighted,
    Disabled,
}

impl Default for BasicData {
    fn default() -> Self {
        Self {
            item_ids: None,
            title: "".to_string(),
            description: vec![],
            notes: vec![],
            acquisition: vec![],
            tags: BTreeMap::new(),
            pinned: false,
            pos: None,
            cached_date: Local::now(),
            href: "".to_string(),
        }
    }
}
