use crate::{addon::Addon, api::gw2_wiki::prepare_item_popup, context::Context};
use nexus::imgui::Ui;
use std::thread;

mod hovered_popup;
mod options;
mod pinned_popup;
pub mod popup_data;
pub mod token_renderer;
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
        self.render_search(ui);
    }

    fn render_progress_indicator(&mut self, ui: &Ui<'_>) {
        if let Some(progress) = self.ui.loading_progress {
            ui.tooltip(|| {
                ui.text(format!("Loading ({}%)", progress));
            });
        }
    }

    fn render_search(&mut self, ui: &Ui) {
        let mut should_focus_input = false;
        if self.should_open_search {
            ui.open_popup("##Search_popup_idp");
            self.should_open_search = false;
            self.search_text = "".to_string();
            should_focus_input = true;
        }
        ui.popup("##Search_popup_idp", || {
            if should_focus_input {
                ui.set_keyboard_focus_here();
            }
            ui.input_text("##search_input_idp", &mut self.search_text)
                .build();
            ui.text_disabled("Press enter to search");
            if ui.is_key_released(nexus::imgui::Key::Enter) {
                ui.close_current_popup();
                Addon::lock_threads().push(thread::spawn(move || {
                    Addon::write_context().ui.loading_progress = Some(1);
                    let item_name = Addon::read_context().search_text.clone();
                    Addon::write_context().search_text = "".to_string();
                    Addon::write_context().ui.hovered_popup =
                        Some(prepare_item_popup(item_name.as_str()));
                    Addon::write_context().ui.loading_progress = None;
                }));
            }
        });
    }
}
