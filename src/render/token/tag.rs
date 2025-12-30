use crate::configuration::popup::rendering_params::RenderingParams;
use crate::render::ui::{UiAction, UiLink};
use crate::state::context::Context;
use crate::state::popup::tag_params::TagParams;
use nexus::imgui::{MouseButton, Ui};

impl Context {
    pub fn render_tag(
        ui: &Ui,
        tag_params: &TagParams,
        pinned: &mut bool,
        ui_actions: &mut Vec<UiAction>,
        current_indent: i32,
        rendering_params: &RenderingParams,
    ) {
        let href = tag_params.href.to_string();
        let title = tag_params.title.to_string();
        Self::render_words(
            ui,
            &tag_params.text,
            current_indent,
            rendering_params,
            |ui: &Ui<'_>, word| {
                ui.text_colored(rendering_params.link_color, word);
                if ui.is_item_hovered() && ui.is_mouse_released(MouseButton::Left) && *pinned {
                    ui_actions.push(UiAction::Open(UiLink {
                        title: title.clone(),
                        href: href.clone(),
                    }));
                }
            },
        );
    }
}
