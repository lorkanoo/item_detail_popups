use std::borrow::Cow;

use nexus::imgui::sys::{self, igGetMousePos};
use nexus::imgui::{ColorEdit, ColorPreview, ComboBoxFlags, MouseButton, SelectableFlags, Ui, Selectable};
use crate::context::Font;

pub trait UiExtended {
    fn mouse_in_bounds(&self, point_min: [f32; 2], point_max: [f32; 2]) -> bool;
    fn input_color_alpha(&self, ui: &Ui, label: impl AsRef<str>, color: &mut [f32; 4]) -> bool;
    fn selected_file<L: AsRef<str>, F: Fn()>(&self, title: L, label: L, buf: &mut String, func: F);
    fn link<T: AsRef<str>>(&self, link: &str, text: T, color: [f32; 4], inline: bool);
    fn font_select(&self, label: impl AsRef<str>, current: &mut Option<Font>) -> bool;
}

impl UiExtended for Ui<'_> {
    fn mouse_in_bounds(&self, bounds_min: [f32; 2], bounds_max: [f32; 2]) -> bool {
        let mut mouse_pos = sys::ImVec2::zero();
        unsafe { igGetMousePos(&mut mouse_pos) };
        bounds_min[0] < mouse_pos.x
            && bounds_min[1] < mouse_pos.y
            && mouse_pos.x < bounds_max[0]
            && mouse_pos.y < bounds_max[1]
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

        if let Some(_token) = self.begin_combo_with_flags(label, preview, ComboBoxFlags::HEIGHT_LARGE) {
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
}
