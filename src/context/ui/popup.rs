use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::render::util::ui::{Linkable, UiElement};
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

impl UiElement for Popup {
    fn rename(&mut self, new_name: String) {
        self.data.title = new_name;
    }

    fn name(&self) -> &String {
        &self.data.title
    }

    fn id(&self) -> &u64 {
        &self.id
    }

    fn pos(&self) -> &Option<[f32; 2]> {
        &self.pos
    }
}

impl Linkable for Popup {
    fn href(&self) -> &String {
        &self.data.href
    }

    fn redirection_href(&self) -> &Option<String> {
        &self.data.redirection_href
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopupData {
    pub item_ids: Option<Vec<u32>>,
    pub title: String,
    pub description: Vec<Token>,
    pub notes: Vec<Token>,
    pub acquisition: Vec<Token>,
    pub images: Vec<Token>,
    // tag href, tag name
    pub tags: BTreeMap<String, String>,
    pub cached_date: DateTime<Local>,
    pub href: String,
    pub redirection_href: Option<String>,
}

impl PopupData {
    pub fn is_not_empty(&self) -> bool {
        !self.description.is_empty()
            || !self.notes.is_empty()
            || !self.acquisition.is_empty()
            || !self.images.is_empty()
            || self.item_ids.is_some()
    }
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
    Image(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Style {
    Normal,
    Highlighted,
    Disabled,
}

impl Default for PopupData {
    fn default() -> Self {
        Self {
            item_ids: None,
            title: "".to_string(),
            description: vec![],
            notes: vec![],
            acquisition: vec![],
            images: vec![],
            tags: BTreeMap::new(),
            cached_date: Local::now(),
            href: "".to_string(),
            redirection_href: None,
        }
    }
}
