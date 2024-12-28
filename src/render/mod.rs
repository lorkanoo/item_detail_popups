use crate::context::Context;
use nexus::imgui::Ui;

mod hovered_popup;
mod options;
mod pinned_popup;
pub mod popup_data;
pub mod util;

impl Context {
    pub fn render(&mut self, ui: &Ui) {
        if !self.run_background_thread {
            return;
        }
        self.render_progress_indicator(ui);
        self.render_popups(ui);
    }

    fn render_popups(&mut self, ui: &Ui) {
        self.render_hovered_popup(ui);
        self.render_pinned_popups(ui);
    }

    fn render_progress_indicator(&mut self, ui: &Ui<'_>) {
        if let Some(progress) = self.ui.loading_progress {
            ui.tooltip(|| {
                ui.text(format!("Loading ({}%)", progress));
            });
        }
    }
}
