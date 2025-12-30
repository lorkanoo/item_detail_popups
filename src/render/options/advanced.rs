use crate::configuration::write_config;
use crate::state::context::Context;
use nexus::imgui::Ui;

impl Context {
    pub fn render_advanced_options(&mut self, ui: &Ui) {
        ui.checkbox(
            "Wait until all keys are released before opening the popup##idp",
            &mut write_config().wait_until_all_keys_released,
        );
        ui.same_line();
        ui.text_disabled("(?)");
        if ui.is_item_hovered() {
            ui.tooltip(|| {
                ui.text("Disabling may cause popups not opening.");
            });
        }
        ui.checkbox(
            "Close popup when mouse moves away##idp",
            &mut write_config().close_on_mouse_away,
        );
        ui.checkbox(
            "Pin on tab hover##idp",
            &mut write_config().rendering_params.auto_pin_on_tab_hover,
        );
        ui.checkbox(
            "Collapse popups on title click##idp",
            &mut write_config().rendering_params.allow_popup_collapsing,
        );
    }
}
