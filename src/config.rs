use crate::addon::VERSION;
use crate::cache::Persistent;

use log::{info, warn};
use nexus::paths::{get_addon_dir, get_game_dir};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS: u64 = 36 * 3600;
const DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS: usize = 200;
const DEFAULT_PRICE_EXPIRATION_DURATION: Duration = Duration::from_secs(60);
const DEFAULT_TEXTURE_EXPIRATION_DURATION: Duration = Duration::from_secs(1 * 24 * 60 * 60);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    pub max_popup_data_cache_elements: usize,
    pub max_popup_data_cache_expiration_duration: Duration,
    pub texture_expiration_duration: Duration,
    #[serde(default = "default_price_expiration")]
    pub price_expiration_duration: Duration, 
}

impl Persistent for Config {
    fn load(&mut self) {
        let path = Config::file_path();
        let file_opt = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok();
        if file_opt.is_none()
        {
            warn!("[load] Failed to load config from \"{}\"", path.display());
            return;
        }
        let reader = BufReader::new(file_opt.unwrap());
        let config_opt = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse config: {err}"))
            .ok();
        if let Some(config) = config_opt {
            *self = config;
        }
        info!("[load] Loaded config from \"{}\"", path.display());
    }

    fn save(&self) {
        let path = Config::file_path();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self).expect("failed to serialize config");
                info!("[save] Saved config to \"{}\"", path.display())
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    fn file_path() -> PathBuf {
        config_dir().join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            max_popup_data_cache_elements: DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS,
            max_popup_data_cache_expiration_duration: Duration::from_secs(
                DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS,
            ),
            price_expiration_duration: DEFAULT_PRICE_EXPIRATION_DURATION,
            texture_expiration_duration: DEFAULT_TEXTURE_EXPIRATION_DURATION,
        }
    }
}

pub trait SwitchValue<T> {
    fn switch(&mut self);
}

pub fn config_dir() -> PathBuf {
    get_addon_dir("item_detail_popups").expect("invalid config directory")
}

pub fn textures_dir() -> PathBuf {
    let mut result = get_addon_dir("item_detail_popups").expect("invalid config directory");
    result.push("textures");
    result
}

pub fn game_dir() -> PathBuf {
    get_game_dir().expect("invalid game directory")
}

fn default_version() -> String {
    VERSION.to_string()
}

fn default_price_expiration() -> Duration {
    DEFAULT_PRICE_EXPIRATION_DURATION
}
