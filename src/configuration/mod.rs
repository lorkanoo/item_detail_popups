pub mod popup {
    pub mod rendering_params;
}
pub mod keyboard_layout;
pub mod notification_params;
pub(crate) mod search;

use crate::addon::PACKAGE_VERSION;
use crate::state::cache::Persist;
use std::fs;
use std::thread;
use crate::configuration::keyboard_layout::KeyboardLayout;
use crate::configuration::notification_params::NotificationParams;
use crate::configuration::popup::rendering_params::RenderingParams;
use crate::state::context::write_context;
use crate::utils::serde::{no, yes};
use log::{info, trace, warn};
use nexus::paths::get_addon_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;
use search::search_params::SearchParams;

pub const GW2_API_KEY: &str = "GW2_API_KEY";
pub const DEFAULT_POST_KEY_COMBINATION_DELAY_MS: u64 = 50;
const DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS: u64 = 36 * 3600;
const DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS: usize = 300;
const DEFAULT_PRICE_EXPIRATION_DURATION: Duration = Duration::from_secs(60);
const DEFAULT_TEXTURE_EXPIRATION_DURATION: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const DEFAULT_BOLD_FONT_NAME: &str = "IDP_default_bold";

pub(crate) static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {

    #[serde(default = "default_version")]
    pub version: String,

    pub max_popup_data_elements: usize,
    pub max_popup_data_expiration_duration: Duration,
    pub max_texture_expiration_duration: Duration,

    #[serde(default = "default_price_expiration")]
    pub max_price_expiration_duration: Duration,

    #[serde(default = "default_bold_font_name")]
    pub selected_bold_font_name: Option<String>,

    #[serde(default = "yes")]
    pub wait_until_all_keys_released: bool,

    #[serde(default = "no")]
    pub use_left_shift: bool,

    #[serde(default = "default_post_key_combination_delay_ms")]
    pub post_key_combination_delay_ms: u64,

    #[serde(default = "yes")]
    pub close_on_mouse_away: bool,

    #[serde(default = "RenderingParams::default")]
    pub rendering_params: RenderingParams,

    #[serde(default = "KeyboardLayout::default")]
    pub keyboard_layout: KeyboardLayout,

    #[serde(default = "NotificationParams::default")]
    pub notification_params: NotificationParams,

    #[serde(default = "SearchParams::default")]
    pub search_params: SearchParams,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: PACKAGE_VERSION.to_string(),
            max_popup_data_elements: DEFAULT_MAX_POPUP_DATA_CACHE_ELEMENTS,
            max_popup_data_expiration_duration: Duration::from_secs(
                DEFAULT_POPUP_DATA_CACHE_EXPIRATION_SECS,
            ),
            max_price_expiration_duration: DEFAULT_PRICE_EXPIRATION_DURATION,
            max_texture_expiration_duration: DEFAULT_TEXTURE_EXPIRATION_DURATION,
            selected_bold_font_name: default_bold_font_name(),
            wait_until_all_keys_released: yes(),
            use_left_shift: no(),
            post_key_combination_delay_ms: DEFAULT_POST_KEY_COMBINATION_DELAY_MS,
            close_on_mouse_away: yes(),
            rendering_params: RenderingParams::default(),
            keyboard_layout: KeyboardLayout::default(),
            notification_params: NotificationParams::default(),
            search_params: SearchParams::default(),
        }
    }
}

impl Persist for Config {
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
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    fn file_path() -> PathBuf {
        config_dir().join("config.json")
    }
}

pub fn write_config() -> RwLockWriteGuard<'static, Config> {
    trace!(
        "[write_config] Acquiring lock (thread {:?})",
        thread::current().id()
    );
    let result = CONFIG
        .get_or_init(|| RwLock::new(Config::default()))
        .write()
        .unwrap();
    trace!(
        "[write_config] Lock acquired (thread {:?})",
        thread::current().id()
    );
    result
}

pub fn read_config() -> RwLockReadGuard<'static, Config> {
    trace!(
        "[read_config] Acquiring lock (thread {:?})",
        thread::current().id()
    );
    let result = CONFIG
        .get_or_init(|| RwLock::new(Config::default()))
        .read()
        .unwrap();
    trace!(
        "[read_config] Lock acquired (thread {:?})",
        thread::current().id()
    );
    result
}

pub(crate) fn load_config_files() {
    let _ = fs::create_dir(config_dir());
    {
        write_config().load();
        write_context().cache.popup_data_map.load();
        write_context().cache.item_names.load();
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
    let mut result = get_addon_dir("item_detail_popups").expect("invalid fonts directory");
    result.push("..\\..\\fonts");
    result
}

fn default_version() -> String {
    PACKAGE_VERSION.to_string()
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
