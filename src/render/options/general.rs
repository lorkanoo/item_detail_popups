use crate::configuration::read_config;
use crate::configuration::GW2_API_KEY;
use crate::render::ui::UiExtended;
use crate::service::credential_manager::store_password;
use crate::state::context::Context;
use nexus::imgui::Ui;

impl Context {
    pub fn render_general_options(&mut self, ui: &Ui) {
        ui.text("GW2 API Key");
        ui.same_line();
        ui.text_disabled("(optional)");
        ui.input_text("##gw2_api_key_input_idp", &mut self.ui.gw2_api_key_input)
            .build();
        ui.same_line();
        if self.ui.gw2_api_key_input.is_empty() {
            ui.text_disabled("Save")
        } else if ui.button("Save##idp") {
            store_password(GW2_API_KEY, &self.ui.gw2_api_key_input);
            self.ui.gw2_api_key_input = "".to_string();
        }
        ui.text_disabled("API key must have inventory access permissions.");
        ui.text_disabled("It is used to check if item is present on any of the characters.");
        ui.link(
            "https://account.arena.net/applications/create",
            "Create your API key here.",
            read_config().rendering_params.link_color,
            false,
        );
    }
}
