use crate::configuration::load_config_files;
use crate::state::context::write_context;
use crate::state::keybinds::{register_open_search_keybind, register_show_popup_keybind};
use crate::threads::{init_threads, unload_threads};
use log::info;
use nexus::gui::{register_render, RenderType};

pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn load() {
    info!("[load] Loading {PACKAGE_NAME} v{PACKAGE_VERSION}");
    load_config_files();
    init_threads();
    register_renderers();
    register_show_popup_keybind();
    register_open_search_keybind();
    info!("[load] {PACKAGE_NAME} loaded");
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
    info!("[unload] Unloading {PACKAGE_NAME} v{PACKAGE_VERSION}");
    unload_threads();
    info!("[unload] {PACKAGE_NAME} unloaded");
}
