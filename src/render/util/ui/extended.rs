use nexus::imgui::sys::{self, igGetMousePos};
use nexus::imgui::{ColorEdit, ColorPreview, Ui};

pub trait UiExtended {
    fn mouse_in_bounds(&self, point_min: [f32; 2], point_max: [f32; 2]) -> bool;
    fn input_color_alpha(&self, ui: &Ui, label: impl AsRef<str>, color: &mut [f32; 4]) -> bool;
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
}
