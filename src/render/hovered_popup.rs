use nexus::imgui::Ui;

use crate::{addon::Addon, context::Context};

use super::util::ui::{extended::UiExtended, UiAction};

impl Context {
    pub fn render_hovered_popup(&mut self, ui: &Ui) {
        let mut ui_actions: Vec<UiAction> = vec![];
        if let Some(popup) = self.ui.hovered_popup.as_mut() {
            if !popup.opened {
                ui.open_popup("##popup_idp");
                popup.opened = true;
            }
            ui.popup("##popup_idp", || {
                ui.group(|| {
                    Self::render_popup_data(
                        ui,
                        None,
                        popup,
                        &mut ui_actions,
                        &mut self.cache,
                        &self.bold_font,
                    );
                });
                if Addon::read_config().close_on_mouse_away {
                    Self::close_popup_on_mouse_away(ui, &mut ui_actions);
                }
                if ui.is_item_clicked() && !popup.pinned {
                    Self::pin_popup(ui, &mut popup.pinned, &mut popup.pos, &mut ui_actions);
                }
            });
        }
        self.process_hovered_popup_actions(ui_actions);
    }

    fn process_hovered_popup_actions(&mut self, ui_actions: Vec<UiAction>) {
        for ui_action in &ui_actions {
            match ui_action {
                UiAction::Pin => {
                    let popup = self.ui.hovered_popup.take().unwrap();
                    self.ui.pinned_popups.push(popup);
                }
                UiAction::Close => {
                    self.ui.hovered_popup = None;
                }
                _ => {}
            }
        }
    }

    fn close_popup_on_mouse_away(ui: &Ui, ui_actions: &mut Vec<UiAction>) {
        let mut hover_bounds_min = ui.window_pos();
        hover_bounds_min[0] -= 25.0;
        hover_bounds_min[1] -= 20.0;
        let mut hover_bounds_max = ui.window_pos();
        let size = ui.window_size();
        hover_bounds_max[0] += size[0] + 15.0;
        hover_bounds_max[1] += size[1] + 15.0;
        if !ui.mouse_in_bounds(hover_bounds_min, hover_bounds_max) {
            ui.close_current_popup();
            ui_actions.push(UiAction::Close);
        }
    }
}
