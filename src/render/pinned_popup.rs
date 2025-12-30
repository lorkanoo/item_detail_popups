use crate::render::ui::{UiAction, CLOSE_BUTTON_MARGIN_OUTER_X, CLOSE_BUTTON_SIZE};
use crate::state::cache::Cache;
use crate::state::context::Context;
use crate::state::font::Font;
use crate::state::popup::{dimensions::Dimensions, popup_state::PopupState, Popup};
use crate::state::threads::link::open_link_thread;
use crate::state::threads::popup::refresh_popup_thread;
use nexus::imgui::{Condition, Ui, Window};

impl Context {
    pub fn render_pinned_popups(&mut self, ui: &Ui) {
        let mut ui_actions = vec![];
        for (pinned_popup_index, popup) in self.ui.pinned_popups.iter_mut().enumerate() {
            Self::render_pinned_popup(
                ui,
                &mut ui_actions,
                pinned_popup_index,
                popup,
                &mut self.cache,
                &self.ui.bold_font,
            );
        }
        self.process_pinned_popups_actions(ui_actions);
    }

    pub fn pin_popup(ui: &Ui, popup_state: &mut PopupState, ui_actions: &mut Vec<UiAction>) {
        ui.close_current_popup();
        ui_actions.push(UiAction::Pin);
        popup_state.pinned = true;
        popup_state.pos = Some(ui.window_pos());
    }

    fn render_pinned_popup(
        ui: &Ui<'_>,
        ui_actions: &mut Vec<UiAction>,
        pinned_popup_index: usize,
        popup: &mut Popup,
        cache: &mut Cache,
        bold_font: &Option<Font>,
    ) {
        let title_text_size = ui.calc_text_size(&popup.data.title);
        let screen_height = ui.io().display_size[1];
        let mut is_opened = popup.state.opened;
        let title_image_width = Dimensions::medium().width;
        let additional_title_width = 30.0;
        Window::new(format!("##idp{}", popup.state.id))
            .position(popup.state.pos.unwrap_or([0.0, 0.0]), Condition::Appearing)
            .always_auto_resize(true)
            .save_settings(false)
            .opened(&mut is_opened)
            .title_bar(false)
            .size_constraints(
                [
                    (title_text_size[0]
                        + title_image_width
                        + CLOSE_BUTTON_SIZE
                        + CLOSE_BUTTON_MARGIN_OUTER_X
                        + additional_title_width),
                    title_text_size[1],
                ],
                [f32::MAX, screen_height],
            )
            .build(ui, || {
                Self::render_popup_data(
                    ui,
                    Some(pinned_popup_index),
                    popup,
                    ui_actions,
                    cache,
                    bold_font,
                );
            });
        if !popup.state.opened {
            ui_actions.push(UiAction::Delete(pinned_popup_index));
        }
    }

    fn process_pinned_popups_actions(&mut self, ui_actions: Vec<UiAction>) {
        let vec = &mut self.ui.pinned_popups;
        for action in &ui_actions {
            match action {
                UiAction::Delete(i) => {
                    if vec.len() > *i {
                        vec.remove(*i);
                    }
                }
                UiAction::Refresh(i) => {
                    if let Some(t) = vec.get(*i) {
                        self.cache.popup_data_map.swap_remove(&t.data.href.clone());
                        if let Some(href) = &t.data.redirection_href {
                            self.cache.popup_data_map.swap_remove(&href.clone());
                        }
                        refresh_popup_thread(t.state.clone(), t.data.title.clone());
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
