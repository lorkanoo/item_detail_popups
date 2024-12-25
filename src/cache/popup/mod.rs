use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::addon::Addon;
use crate::context::ui::popup::Popup;
use chrono::Local;
use function_name::named;
use indexmap::IndexMap;
use log::info;
use crate::cache::Cache;
use crate::config::config_dir;

impl Cache {
    pub fn add_popup(&mut self, key: &str, popup: &mut Popup, refresh_date: bool) {
        let max_popup_cache_size = Addon::lock().config.max_popup_cache_size;
        while self.popups.len() >= max_popup_cache_size {
            if self.popups.shift_remove_index(0).is_none() {
                break;
            }
        }
        if max_popup_cache_size > 0 {
            if refresh_date {
                popup.basic_data.cached_date = Local::now();
            }
            self.popups.insert(key.to_owned(), popup.clone());
        }
    }

    #[named]
    pub fn try_load_popups() -> Option<IndexMap<String, Popup>> {
        let path = Self::popups_file();
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
    pub fn save_popups(popups: &IndexMap<String, Popup>) {
        let path = Self::popups_file();
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

    pub fn popups_file() -> PathBuf {
        config_dir().join("popup_cache.json")
    }
}
