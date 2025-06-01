use crate::context::{font::Font, ui::popup::dimensions::Dimensions};
use nexus::imgui::{Condition, Ui, Window};

use crate::{
    cache::Cache,
    context::{ui::popup::Popup, Context},
    thread::{open_link_thread, refresh_popup_thread},
};

use super::util::ui::{
    extended::{CLOSE_BUTTON_MARGIN_OUTER_X, CLOSE_BUTTON_SIZE},
    UiAction,
};

impl Context {
    pub fn render_pinned_popups(&mut self, ui: &Ui) {
        let mut ui_actions = vec![];
        for (i, popup) in self.ui.pinned_popups.iter_mut().enumerate() {
            Self::render_pinned_popup(
                ui,
                &mut ui_actions,
                i,
                popup,
                &mut self.cache,
                &self.bold_font,
            );
        }
        self.process_pinned_popups_actions(ui_actions);
    }

    pub fn pin_popup(
        ui: &Ui,
        popup_pinned: &mut bool,
        popup_pos: &mut Option<[f32; 2]>,
        ui_actions: &mut Vec<UiAction>,
    ) {
        ui.close_current_popup();
        ui_actions.push(UiAction::Pin);
        *popup_pinned = true;
        *popup_pos = Some(ui.window_pos());
    }

    fn render_pinned_popup(
        ui: &Ui<'_>,
        ui_actions: &mut Vec<UiAction>,
        popup_vec_index: usize,
        popup: &mut Popup,
        cache: &mut Cache,
        bold_font: &Option<Font>,
    ) {
        let title_text_size = ui.calc_text_size(&popup.data.title);
        let screen_height = ui.io().display_size[1];
        let mut is_opened = popup.opened;
        let title_image_width = Dimensions::medium().width;
        let additional_title_width = 30.0;
        Window::new(format!("##idp{}", popup.id))
            .position(popup.pos.unwrap_or([0.0, 0.0]), Condition::Appearing)
            .always_auto_resize(true)
            .save_settings(false)
            .opened(&mut is_opened)
            .title_bar(false)
            .size_constraints(
                [
                    (&title_text_size[0]
                        + title_image_width
                        + CLOSE_BUTTON_SIZE
                        + CLOSE_BUTTON_MARGIN_OUTER_X
                        + additional_title_width),
                    title_text_size[1],
                ],
                [f32::MAX, screen_height * 0.5],
            )
            .build(ui, || {
                Self::render_popup_data(
                    ui,
                    Some(popup_vec_index),
                    popup,
                    ui_actions,
                    cache,
                    bold_font,
                );
            });
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
