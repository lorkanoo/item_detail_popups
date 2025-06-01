use crate::addon::VERSION;
use crate::cache::Persistent;

use keyboard_layout::KeyboardLayout;
use log::{info, warn};
use nexus::paths::{get_addon_dir, get_game_dir};
use rendering_params::RenderingParams;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::Duration;

pub mod keyboard_layout;
pub mod rendering_params;
pub const DEFAULT_POST_KEY_COMBINATION_DELAY_MS: u64 = 50;
const DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS: u64 = 36 * 3600;
const DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS: usize = 300;
const DEFAULT_PRICE_EXPIRATION_DURATION: Duration = Duration::from_secs(60);
const DEFAULT_TEXTURE_EXPIRATION_DURATION: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const DEFAULT_BOLD_FONT_NAME: &str = "IDP_default_bold";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    pub max_popup_data_cache_elements: usize,
    pub max_popup_data_cache_expiration_duration: Duration,
    pub max_texture_expiration_duration: Duration,
    #[serde(default = "default_price_expiration")]
    pub max_price_expiration_duration: Duration,
    #[serde(default = "default_bold_font_name")]
    pub selected_bold_font_name: Option<String>,
    #[serde(default = "yes")]
    pub wait_until_all_keys_released: bool,
    #[serde(default = "default_post_key_combination_delay_ms")]
    pub post_key_combination_delay_ms: u64,
    #[serde(default = "yes")]
    pub close_on_mouse_away: bool,
    #[serde(default = "RenderingParams::default")]
    pub rendering_params: RenderingParams,
    #[serde(default = "KeyboardLayout::default")]
    pub keyboard_layout: KeyboardLayout,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            max_popup_data_cache_elements: DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS,
            max_popup_data_cache_expiration_duration: Duration::from_secs(
                DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS,
            ),
            max_price_expiration_duration: DEFAULT_PRICE_EXPIRATION_DURATION,
            max_texture_expiration_duration: DEFAULT_TEXTURE_EXPIRATION_DURATION,
            selected_bold_font_name: default_bold_font_name(),
            wait_until_all_keys_released: yes(),
            post_key_combination_delay_ms: 50,
            close_on_mouse_away: yes(),
            rendering_params: RenderingParams::default(),
            keyboard_layout: KeyboardLayout::default(),
        }
    }
}

impl Persistent for Config {
    fn load(&mut self) {
        let path = Config::file_path();
        let file_opt = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok();
        if file_opt.is_none() {
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

pub fn config_dir() -> PathBuf {
    get_addon_dir("item_detail_popups").expect("invalid config directory")
}

pub fn textures_dir() -> PathBuf {
    let mut result = get_addon_dir("item_detail_popups").expect("invalid config directory");
    result.push("textures");
    result
}

pub fn fonts_dir() -> PathBuf {
    let mut result = config_dir();
    result.push("fonts");
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

fn default_bold_font_name() -> Option<String> {
    Some(DEFAULT_BOLD_FONT_NAME.to_string())
}

fn default_post_key_combination_delay_ms() -> u64 {
    DEFAULT_POST_KEY_COMBINATION_DELAY_MS
}

pub fn no() -> bool {
    false
}

pub fn yes() -> bool {
    true
}
