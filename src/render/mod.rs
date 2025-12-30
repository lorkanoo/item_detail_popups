use crate::state::context::Context;
use log::debug;
use nexus::imgui::Ui;
use crate::addon::PACKAGE_NAME;

mod hovered_popup;
mod options;
mod pinned_popup;
pub mod popup_data;
pub mod token;
pub mod ui;
mod search;

impl Context {
    pub fn render(&mut self, ui: &Ui) {
        debug!("[render]");
        let _ = ui.push_id(PACKAGE_NAME);
        if !self.run_background_thread {
            return;
        }
        self.render_progress_indicator(ui);
        self.render_popups(ui);
    }

    fn render_popups(&mut self, ui: &Ui) {
        debug!("[render_popups]");
        self.render_hovered_popup(ui);
        self.render_pinned_popups(ui);
        self.render_search_prompt(ui);
        self.render_search_result(ui);
    }

    fn render_progress_indicator(&mut self, ui: &Ui<'_>) {
        debug!("[render_progress_indicator]");
        if let Some(progress) = self.ui.loading_progress {
            ui.tooltip(|| {
                ui.text(format!("Loading ({}%)", progress));
            });
        }
    }
}
