use nexus::imgui::{Condition, Ui, Window};
use crate::context::Font;

use crate::{
    addon::Addon, cache::Cache, context::{ui::popup::Popup, Context}, thread::{open_link_thread, refresh_popup_thread}
};

use super::util::ui::UiAction;

impl Context {
    pub fn render_pinned_popups(&mut self, ui: &Ui) {
        let mut ui_actions = vec![];
        for (i, popup) in self.ui.pinned_popups.iter_mut().enumerate() {
            Self::render_pinned_popup(ui, &mut ui_actions, i, popup, &mut self.cache, &self.bold_font);
        }
        self.process_pinned_popups_actions(ui_actions);
    }

    pub fn pin_popup(ui: &Ui, popup: &mut Popup, ui_actions: &mut Vec<UiAction>) {
        ui.close_current_popup();
        ui_actions.push(UiAction::Pin);
        popup.pinned = true;
        popup.pos = Some(ui.window_pos());
    }

    fn render_pinned_popup(
        ui: &Ui<'_>,
        ui_actions: &mut Vec<UiAction>,
        popup_vec_index: usize,
        popup: &mut Popup,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        let size = ui.calc_text_size(&popup.data.title);
        let screen_height = ui.io().display_size[1];
        let mut is_opened = popup.opened;
        Window::new(format!("{}##idp{}", popup.data.title.clone(), popup.id))
            .position(popup.pos.unwrap_or([0.0, 0.0]), Condition::Appearing)
            .collapsible(Addon::read_config().allow_collapsing_popups)
            .always_auto_resize(true)
            .save_settings(false)
            .opened(&mut is_opened)
            .size_constraints(
                [&size[0] * 1.25, &size[1] * 1.0],
                [f32::MAX, screen_height * 0.5],
            )
            .build(ui, || {
                Self::render_popup_data(
                    ui,
                    Some(popup_vec_index),
                    popup,
                    ui_actions,
                    *ui.window_pos().first().unwrap() + 640.0,
                    cache,
                    bold_font
                );
            });
        popup.opened = is_opened;
        if !popup.opened {
            ui_actions.push(UiAction::Delete(popup_vec_index));
        }
    }

    fn process_pinned_popups_actions(&mut self, ui_actions: Vec<UiAction>) {
        let vec = &mut self.ui.pinned_popups;
        for action in &ui_actions {
            match action {
                UiAction::Delete(i) => {
                    vec.remove(*i);
                }
                UiAction::Refresh(i) => {
                    if let Some(t) = vec.get(*i) {
                        self.cache.popup_data_map.swap_remove(&t.data.href.clone());
                        if let Some(href) = &t.data.redirection_href {
                            self.cache.popup_data_map.swap_remove(&href.clone());
                        }
                        refresh_popup_thread(t.id, t.data.title.clone(), t.pos);
                        vec.remove(*i);
                    }
                }
                UiAction::Open(ui_link) => {
                    open_link_thread(ui_link.href.clone(), ui_link.title.clone())
                }
                _ => {}
            }
        }
    }
}
