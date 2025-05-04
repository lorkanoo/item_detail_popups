mod config;
mod context;
mod keybind;
mod threads;

use crate::cache::{Cache, Persistent};
use crate::config::{config_dir, Config};
use crate::context::Context;

use log::info;
use nexus::gui::{register_render, RenderType};
use std::fs;
use std::sync::{Mutex, OnceLock, RwLock};
use std::thread::JoinHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
static MULTITHREADED_ADDON: Addon = Addon {
    context: OnceLock::new(),
    config: OnceLock::new(),
    threads: OnceLock::new(),
};

pub struct Addon {
    context: OnceLock<RwLock<Context>>,
    config: OnceLock<RwLock<Config>>,
    threads: OnceLock<Mutex<Vec<JoinHandle<()>>>>,
}

impl Addon {
    pub fn load() {
        info!("[load] Loading item_detail_popups v{}", VERSION);
        Self::load_config_files();
        Self::init_threads();
        Self::register_renderers();
        Self::register_show_popup_keybind();
        Self::register_open_search_keybind();
        info!("[load] item_detail_popups loaded");
    }

    fn register_renderers() {
        register_render(
            RenderType::Render,
            nexus::gui::render!(|ui| Addon::write_context().render(ui)),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::write_context().render_options(ui)),
        )
        .revert_on_unload();
    }

    fn load_config_files() {
        let _ = fs::create_dir(config_dir());
        {
            Addon::write_config().load();
            Addon::write_context().cache.popup_data_map.load();
            Addon::write_context().cache.item_names.load();
        }
    }

    pub fn unload() {
        info!("[unload] Unloading item_detail_popups v{VERSION}");
        Self::unload_threads();
        Self::save_config();
        Self::save_cache();
        info!("[unload] item_detail_popups unloaded");
    }

    fn save_config() {
        info!("[save_config] Saving configuration..");
        Self::read_config().save();
    }

    fn save_cache() {
        Self::read_context().cache.item_names.save();
        Self::read_context().cache.popup_data_map.save();
    }
}
