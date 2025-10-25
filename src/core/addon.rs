use crate::configuration::config::{load_config_files, read_config};
use crate::core::threads::{init_threads, unload_threads};
use crate::state::cache::cache::Persist;
use crate::state::context::{save_cache, write_context};
use crate::state::keybinds::{register_open_search_keybind, register_show_popup_keybind};
use log::info;
use nexus::gui::{register_render, RenderType};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn load() {
    info!("[load] Loading item_detail_popups v{}", VERSION);
    load_config_files();
    init_threads();
    register_renderers();
    register_show_popup_keybind();
    register_open_search_keybind();
    info!("[load] item_detail_popups loaded");
}

fn register_renderers() {
    register_render(
        RenderType::Render,
        nexus::gui::render!(|ui| write_context().render(ui)),
    )
    .revert_on_unload();

    register_render(
        RenderType::OptionsRender,
        nexus::gui::render!(|ui| write_context().render_options(ui)),
    )
    .revert_on_unload();
}

pub fn unload() {
    info!("[unload] Unloading item_detail_popups v{VERSION}");
    unload_threads();
    read_config().save();
    save_cache();
    info!("[unload] item_detail_popups unloaded");
}
