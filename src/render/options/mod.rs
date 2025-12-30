mod advanced;
mod cache;
mod general;
mod help;
pub mod r#macro;
mod style;

use crate::state::context::Context;
use log::debug;
use nexus::imgui::Ui;
use crate::addon::PACKAGE_NAME;

impl Context {
    pub fn render_options(&mut self, ui: &Ui) {
        debug!("[render_options] Started.");
        let _ = ui.push_id(format!("${PACKAGE_NAME}_options").as_str());
        if let Some(_token) = ui.tab_bar("options#idp") {
            // if let Some(_token) = ui.tab_item("General") {
            //     self.render_general_options(ui);
            // }
            if let Some(_token) = ui.tab_item("Style") {
                self.render_style_options(ui);
            }
            if let Some(_token) = ui.tab_item("Macro") {
                self.render_macro_options(ui);
            }
            if let Some(_token) = ui.tab_item("Cache") {
                self.render_cache_options(ui);
            }
            if let Some(_token) = ui.tab_item("Advanced") {
                self.render_advanced_options(ui);
            }
            if let Some(_token) = ui.tab_item("Help") {
                self.render_help(ui);
            }
        }
    }
}
