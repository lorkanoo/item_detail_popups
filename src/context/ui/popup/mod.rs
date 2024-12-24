use chrono::{DateTime, Local};
use function_name::named;
use indexmap::IndexMap;
use log::info;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::config_dir;
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

    #[named]
    pub fn try_load() -> Option<IndexMap<String, Popup>> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read popup_cache: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse popup_cache: {err}"))
            .ok()?;
        info!(
            "[{}] Loaded popup_cache from \"{}\"",
            function_name!(),
            path.display()
        );
        Some(config)
    }

    #[named]
    pub fn save(popups: &IndexMap<String, Popup>) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &popups)
                    .expect("failed to serialize popup_cache");
                info!(
                    "[{}] Saved popup_cache to \"{}\"",
                    function_name!(),
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save popup_cache: {err}"),
        }
    }

    pub fn file() -> PathBuf {
        config_dir().join("popup_cache.json")
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
