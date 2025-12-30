use crate::state::context::Context;
use nexus::imgui::Ui;

impl Context {
    pub fn render_list_element(
        ui: &Ui,
        starts_with_list: &mut bool,
        current_indent: i32,
        use_bullet_list_punctuation: bool,
    ) {
        if !*starts_with_list {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
        *starts_with_list = false;
        if use_bullet_list_punctuation {
            ui.text(if current_indent.eq(&0) { "• " } else { "- " });
        } else {
            ui.text("- ");
        }
    }
}
