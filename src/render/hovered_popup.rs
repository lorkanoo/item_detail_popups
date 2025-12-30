use crate::configuration::read_config;
use crate::render::ui::{UiAction, UiExtended};
use crate::state::context::Context;
use log::debug;
use nexus::imgui::Ui;

const CLOSE_POPUP_TOLERANCE_LEFT: f32 = 25.0;
const CLOSE_POPUP_TOLERANCE_RIGHT: f32 = 15.0;
const CLOSE_POPUP_TOLERANCE_TOP: f32 = 20.0;
const CLOSE_POPUP_TOLERANCE_BOTTOM: f32 = 15.0;

impl Context {
    pub fn render_hovered_popup(&mut self, ui: &Ui) {
        debug!("[render_hovered_popup]");
        let mut ui_actions: Vec<UiAction> = vec![];
        if let Some(popup) = self.ui.hovered_popup.as_mut() {
            if !popup.state.opened {
                ui.open_popup("##popup_idp");
                popup.state.opened = true;
            }
            ui.popup("##popup_idp", || {
                ui.group(|| {
                    Self::render_popup_data(
                        ui,
                        None,
                        popup,
                        &mut ui_actions,
                        &mut self.cache,
                        &self.ui.bold_font,
                    );
                });
                if read_config().close_on_mouse_away {
                    Self::close_popup_on_mouse_away(ui, &mut ui_actions);
                }
                if ui.is_item_clicked() && !popup.state.pinned {
                    Self::pin_popup(ui, &mut popup.state, &mut ui_actions);
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
        hover_bounds_min[0] -= CLOSE_POPUP_TOLERANCE_LEFT;
        hover_bounds_min[1] -= CLOSE_POPUP_TOLERANCE_TOP;
        let mut hover_bounds_max = ui.window_pos();
        let size = ui.window_size();
        hover_bounds_max[0] += size[0] + CLOSE_POPUP_TOLERANCE_RIGHT;
        hover_bounds_max[1] += size[1] + CLOSE_POPUP_TOLERANCE_BOTTOM;
        if !ui.mouse_in_bounds(hover_bounds_min, hover_bounds_max) {
            ui.close_current_popup();
            ui_actions.push(UiAction::Close);
        }
    }
}
