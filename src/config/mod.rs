use crate::addon::{Addon, VERSION};
use function_name::named;
use log::info;
use nexus::paths::{get_addon_dir, get_game_dir};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::MutexGuard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    pub max_popup_cache_size: usize,
    //hours, minutes
    pub max_popup_cache_expiration: (i64, i64),
    #[serde(default = "default_price_expiration_sec")]
    pub price_expiration_sec: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            max_popup_cache_size: 500,
            max_popup_cache_expiration: (336, 0),
            price_expiration_sec: default_price_expiration_sec(),
        }
    }
}

impl Config {
    #[named]
    pub fn try_load() -> Option<Self> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse config: {err}"))
            .ok()?;
        info!(
            "[{}] Loaded config from \"{}\"",
            function_name!(),
            path.display()
        );
        Some(config)
    }

    #[named]
    pub fn save(&self) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize config");
                info!(
                    "[{}] Saved config to \"{}\"",
                    function_name!(),
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    pub fn file() -> PathBuf {
        config_dir().join("config.json")
    }

    pub fn valid(&self) -> bool {
        true
    }
}

pub fn config_dir() -> PathBuf {
    get_addon_dir("item_detail_popups").expect("invalid config directory")
}

pub fn game_dir() -> PathBuf {
    get_game_dir().expect("invalid game directory")
}

fn default_version() -> String {
    VERSION.to_string()
}

fn default_price_expiration_sec() -> i64 {
    30
}

pub fn migrate_configs(addon: &mut MutexGuard<Addon>) {
    addon.config.version = VERSION.to_string();
}

#[allow(dead_code)]
fn version_older_than(older: &str, than: &str) -> bool {
    Version::parse(older).unwrap() < Version::parse(than).unwrap()
}

pub trait SwitchValue<T> {
    fn switch(&mut self);
}
