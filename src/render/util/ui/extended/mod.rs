use crate::render::util::ui::UiAction;
use nexus::imgui::sys::igGetMousePos;
use nexus::imgui::{sys, Direction, Slider, SliderFlags, StyleColor, Ui};

#[allow(dead_code)]
pub trait UiExtended {
    fn header<T: AsRef<str>>(&self, text: T);
    fn selected_file<L: AsRef<str>, F: Fn()>(&self, title: L, label: L, buf: &mut String, func: F);
    fn move_up_button(&self, ui_actions: &mut Vec<UiAction>, el_index: usize);
    fn move_down_button(
        &self,
        ui_actions: &mut Vec<UiAction>,
        rule_index: usize,
        last_rule_index: usize,
    );
    fn slider_percent(&self, label: impl AsRef<str>, value: &mut f32) -> bool;

    fn mouse_in_bounds(&self, point_min: [f32; 2], point_max: [f32; 2]) -> bool;
}

impl UiExtended for Ui<'_> {
    fn header<T: AsRef<str>>(&self, text: T) {
        self.text(text);
        self.spacing();
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

    fn move_up_button(&self, ui_actions: &mut Vec<UiAction>, el_index: usize) {
        if el_index != 0 {
            if self.arrow_button(format!("##move_up{}", el_index), Direction::Up) {
                ui_actions.push(UiAction::MoveUp(el_index));
            }
            self.same_line();
        } else {
            let color = self.style_color(StyleColor::TextDisabled);
            let style = self.push_style_color(StyleColor::Text, color);
            self.arrow_button("", Direction::Up);
            style.end();
            self.same_line();
        }
    }

    fn move_down_button(
        &self,
        ui_actions: &mut Vec<UiAction>,
        el_index: usize,
        last_el_index: usize,
    ) {
        if last_el_index == el_index {
            let color = self.style_color(StyleColor::TextDisabled);
            let style = self.push_style_color(StyleColor::Text, color);
            self.arrow_button("", Direction::Down);
            style.end();
        } else if self.arrow_button(format!("##pr_down{}", el_index), Direction::Down) {
            ui_actions.push(UiAction::MoveDown(el_index));
        }
    }

    fn slider_percent(&self, label: impl AsRef<str>, value: &mut f32) -> bool {
        let mut percent = *value * 100.0;
        if Slider::new(label, 0.0, 100.0f32)
            .flags(SliderFlags::ALWAYS_CLAMP)
            .display_format("%.2f")
            .build(self, &mut percent)
        {
            *value = percent / 100.0;
            true
        } else {
            false
        }
    }

    fn mouse_in_bounds(&self, bounds_min: [f32; 2], bounds_max: [f32; 2]) -> bool {
        let mut mouse_pos = sys::ImVec2::zero();
        unsafe { igGetMousePos(&mut mouse_pos) };
        bounds_min[0] < mouse_pos.x
            && bounds_min[1] < mouse_pos.y
            && mouse_pos.x < bounds_max[0] && mouse_pos.y < bounds_max[1]
    }
}
