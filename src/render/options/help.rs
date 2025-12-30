use crate::configuration::read_config;
use crate::render::ui::UiExtended;
use crate::state::context::Context;
use nexus::imgui::Ui;

impl Context {
    pub fn render_help(&mut self, ui: &Ui) {
        ui.text("This addon requires english in-game language to detect items properly.");
        ui.text("In case of problems while using non-QWERTY keyboard, change keyboard layout under 'Macro' settings.");
        ui.text("To ask questions / report issues message me in game (lorkano.4609) or visit");
        ui.link(
            "https://discord.com/channels/410828272679518241/1321117612209602601",
            "discord",
            read_config().rendering_params.link_color,
            true,
        );
        ui.text("channel.");
        ui.text("Please make sure to read");
        ui.link(
            "https://github.com/lorkanoo/item_detail_popups",
            "usage guide",
            read_config().rendering_params.link_color,
            true,
        );
        ui.text("in case of any problems.");
    }
}
