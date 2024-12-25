mod cache;
mod keybind;
mod threads;

use crate::api::gw2tp::fetch_item_names_thread;
use crate::cache::Cache;
use crate::config::{config_dir, migrate_configs, Config};
use crate::context::{init_context, Context};
use crate::thread::background_thread;
use function_name::named;
use log::info;
use nexus::gui::{register_render, RenderType};
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::JoinHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
static MULTITHREADED_ADDON: MultithreadedAddon = MultithreadedAddon {
    addon: OnceLock::new(),
    threads: OnceLock::new(),
    cache: OnceLock::new(),
};

pub struct MultithreadedAddon {
    pub addon: OnceLock<Mutex<Addon>>,
    pub threads: OnceLock<Mutex<Vec<JoinHandle<()>>>>,
    pub cache: OnceLock<Mutex<Cache>>,
}

#[derive(Default)]
pub struct Addon {
    pub config: Config,
    pub context: Context,
    pub cache: Cache,
}

impl Addon {
    pub fn lock() -> MutexGuard<'static, Addon> {
        MULTITHREADED_ADDON
            .addon
            .get_or_init(|| Mutex::new(Addon::default()))
            .lock()
            .unwrap()
    }

    #[named]
    pub fn load() {
        info!(
            "[{}] Loading item_detail_popups v{}",
            function_name!(),
            VERSION
        );
        Self::load_config_files();

        migrate_configs(&mut Addon::lock());
        init_context();
        fetch_item_names_thread();
        background_thread();
        register_render(
            RenderType::Render,
            nexus::gui::render!(|ui| Addon::lock().render(ui)),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        Self::register_show_popup_keybind();
        info!("[{}] item_detail_popups loaded", function_name!());
    }

    fn load_config_files() {
        let _ = fs::create_dir(config_dir());
        {
            if let Some(config) = Config::try_load() {
                Addon::lock().config = config;
            }
            if let Some(popups) = Cache::try_load_popups() {
                Addon::cache().popups = popups;
            }
            if let Some(item_names) = Cache::try_load_item_names() {
                Addon::cache().item_names = item_names;
            }
        }
    }

    #[named]
    pub fn unload() {
        info!(
            "[{}] Unloading item_detail_popups v{VERSION}",
            function_name!()
        );
        Self::unload_threads();
        Self::save_config();
        info!("[{}] item_detail_popups unloaded", function_name!());
    }

    #[named]
    fn save_config() {
        let addon = &mut Self::lock();
        info!("[{}] Saving configuration..", function_name!());
        addon.config.save();
        let cache = &mut Self::cache();
        Cache::save_popups(&cache.popups);
        Cache::save_item_names(&cache.item_names);
    }
}
