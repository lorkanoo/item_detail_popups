use std::borrow::Cow;

use nexus::imgui::sys::{self, ImVec2};
use nexus::imgui::{
    ColorEdit, ColorPreview, ComboBoxFlags, MouseButton, Selectable, SelectableFlags, StyleColor,
    Ui,
};

use crate::state::font::Font;

pub const CLOSE_BUTTON_SIZE: f32 = 25.0;
pub const CLOSE_BUTTON_MARGIN_OUTER_X: f32 = 15.0;

pub const HIGHLIGHT_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

#[derive(Clone, Debug)]
pub enum UiAction {
    Delete(usize),
    Refresh(usize),
    Close,
    Pin,
    Open(UiLink),
}

#[derive(Clone, Debug)]
pub struct UiLink {
    pub title: String,
    pub href: String,
}

#[allow(dead_code)]
pub trait UiExtended {
    fn mouse_in_bounds(&self, point_min: [f32; 2], point_max: [f32; 2]) -> bool;
    fn input_color_alpha(&self, ui: &Ui, label: impl AsRef<str>, color: &mut [f32; 4]) -> bool;
    fn selected_file<L: AsRef<str>, F: Fn()>(&self, title: L, label: L, buf: &mut String, func: F);
    fn link<T: AsRef<str>>(&self, link: &str, text: T, color: [f32; 4], inline: bool);
    fn font_select(&self, label: impl AsRef<str>, current: &mut Option<Font>) -> bool;
    fn text_vert_centered(&self, text: impl AsRef<str>, height: &f32, disabled: &bool);
    fn text_or_disabled(&self, text: impl AsRef<str>, should_render_disabled: &bool);
    fn close_button(&self, text: impl AsRef<str>, x_pos_limit: &f32) -> bool;
    fn not_in_view(&self, height: &f32) -> bool;
    fn set_next_window_pos(&self, pos: [f32; 2]);
}

impl UiExtended for Ui<'_> {
    fn mouse_in_bounds(&self, bounds_min: [f32; 2], bounds_max: [f32; 2]) -> bool {
        let mouse_pos = self.io().mouse_pos;
        bounds_min[0] < mouse_pos[0]
            && bounds_min[1] < mouse_pos[1]
            && mouse_pos[0] < bounds_max[0]
            && mouse_pos[1] < bounds_max[1]
    }

    fn input_color_alpha(&self, ui: &Ui, label: impl AsRef<str>, color: &mut [f32; 4]) -> bool {
        ColorEdit::new(label, color)
            .preview(ColorPreview::Alpha)
            .alpha_bar(true)
            .build(ui)
    }

    fn selected_file<L: AsRef<str>, F: Fn()>(
        &self,
        title: L,
        label: L,
        buf: &mut String,
        on_select: F,
    ) {
        self.text(title);
        self.input_text(&label, buf)
            .hint("File location")
            .auto_select_all(true)
            .read_only(true)
            .build();
        self.same_line();
        if self.button(format!("Select##{}", label.as_ref())) {
            on_select();
        }
    }

    fn link<T: AsRef<str>>(&self, link: &str, text: T, color: [f32; 4], inline: bool) {
        if inline {
            self.same_line();
        }
        self.text_colored(color, text);
        if self.is_item_hovered() && self.is_mouse_released(MouseButton::Left) {
            if let Err(err) = open::that_detached(link) {
                log::error!("Failed to open url: {err}");
            }
        }
        if inline {
            self.same_line();
        }
    }

    fn font_select(&self, label: impl AsRef<str>, current: &mut Option<Font>) -> bool {
        const INHERIT: &str = "Default";

        let mut changed = false;
        let preview = current
            .map(|current| unsafe { current.name_raw() }.to_string_lossy())
            .unwrap_or(Cow::Borrowed(INHERIT));

        if let Some(_token) =
            self.begin_combo_with_flags(label, preview, ComboBoxFlags::HEIGHT_LARGE)
        {
            if Selectable::new(INHERIT).build(self) {
                *current = None;
                changed = true;
            }

            for font in unsafe { Font::get_all() } {
                let is_selected = Some(font) == *current;
                if unsafe {
                    sys::igSelectable_Bool(
                        font.name_raw().as_ptr(),
                        is_selected,
                        SelectableFlags::empty().bits() as i32,
                        [0.0, 0.0].into(),
                    )
                } {
                    *current = Some(font);
                    changed = true;
                }
                if is_selected {
                    self.set_item_default_focus();
                }
            }
        }
        changed
    }

    fn text_vert_centered(&self, text: impl AsRef<str>, height: &f32, disabled: &bool) {
        let text_height = self.calc_text_size(&text)[1];
        let cur_pos = self.cursor_pos();
        self.set_cursor_pos([
            cur_pos[0],
            cur_pos[1] + (height / 2.0) - (text_height / 2.0),
        ]);
        self.text_or_disabled(text, disabled);
    }

    fn text_or_disabled(&self, text: impl AsRef<str>, should_render_disabled: &bool) {
        if *should_render_disabled {
            self.text_disabled(&text);
        } else {
            self.text(&text);
        }
    }

    fn close_button(&self, text: impl AsRef<str>, x_pos_limit: &f32) -> bool {
        let style = self.push_style_var(nexus::imgui::StyleVar::FrameBorderSize(0.0));
        let button_dimension = CLOSE_BUTTON_SIZE;
        let margin_outer_x = CLOSE_BUTTON_MARGIN_OUTER_X;
        let margin_outer_y = 10.0;
        let margin_inner = 4.0;

        let window_size = self.window_size();
        self.set_cursor_pos([
            f32::min(
                window_size[0] - button_dimension - margin_outer_x,
                x_pos_limit - button_dimension - margin_outer_x,
            ),
            margin_outer_y,
        ]);
        let result = self.button_with_size(&text, [button_dimension, button_dimension]);

        let min = self.item_rect_min();
        let min_with_margin = [min[0] + margin_inner, min[1] + margin_inner];

        let max = self.item_rect_max();
        let max_with_margin = [max[0] - margin_inner, max[1] - margin_inner];

        let draw_list = self.get_window_draw_list();
        let color = self.style_color(StyleColor::Text);
        draw_list
            .add_line(min_with_margin, max_with_margin, color)
            .build();

        draw_list
            .add_line(
                [min[0] + margin_inner, max[1] - margin_inner],
                [max[0] - margin_inner, min[1] + margin_inner],
                color,
            )
            .build();

        style.pop();
        result
    }

    fn not_in_view(&self, height: &f32) -> bool {
        let cursor_pos_y = self.cursor_pos()[1];
        cursor_pos_y < self.scroll_y() - height * 2.0
            || cursor_pos_y > self.scroll_y() + self.window_size()[1] + height
    }

    fn set_next_window_pos(&self, pos: [f32; 2]) {
        unsafe { sys::igSetNextWindowPos(ImVec2::new(pos[0], pos[1]), 0.into(), ImVec2::zero()) }
    }
}
