use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::render::util::ui::UiElement;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static POPUP_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));

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
    pub title: String,
    pub description: Vec<Token>,
    pub notes: Vec<Token>,
    // href, item_name
    pub tags: BTreeMap<String, String>,
    pub pinned: bool,
    pub pos: Option<[f32; 2]>,
    pub cached_date: DateTime<Local>,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Token {
    Text(String, Style),
    // href, name
    Tag(String, String),
    ListElement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Style {
    Normal,
    Highlighted,
}

impl Default for BasicData {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            description: vec![],
            notes: vec![],
            tags: BTreeMap::new(),
            pinned: false,
            pos: None,
            cached_date: Local::now(),
            href: "".to_string(),
        }
    }
}
