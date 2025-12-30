use crate::configuration::{read_config, write_config};
use crate::render::ui::UiExtended;
use crate::state::context::Context;
use crate::state::threads::font::load_fonts;
use nexus::imgui::{Key, TreeNodeFlags, Ui};

const ERROR_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

impl Context {
    pub fn render_style_options(&mut self, ui: &Ui) {
        self.render_bold_font_options(ui);
        ui.checkbox(
            "Use bullet character in list punctuation##idp",
            &mut write_config().rendering_params.use_bullet_list_punctuation,
        );
        self.render_max_content_width(ui);
        self.render_max_content_height(ui);
        self.render_color_options(ui);
        self.render_visibility_options(ui);
    }

    fn render_color_options(&mut self, ui: &Ui<'_>) {
        if ui.collapsing_header("Colors##idp", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
            ui.text("Link color:");
            ui.input_color_alpha(
                ui,
                "##idp_link_color",
                &mut write_config().rendering_params.link_color,
            );
            ui.text("Gold coin color:");
            ui.input_color_alpha(
                ui,
                "##idp_gold_coin_color",
                &mut write_config().rendering_params.gold_coin_color,
            );
            ui.text("Silver coin color:");
            ui.input_color_alpha(
                ui,
                "##idp_silver_coin_color",
                &mut write_config().rendering_params.silver_coin_color,
            );
            ui.text("Copper coin color:");
            ui.input_color_alpha(
                ui,
                "##idp_copper_coin_color",
                &mut write_config().rendering_params.copper_coin_color,
            );
        }
    }

    fn render_bold_font_options(&mut self, ui: &Ui) {
        ui.text("Bold font");
        ui.same_line();
        ui.text_disabled(" (place fonts under 'addons/item_detail_popups/fonts')");

        if ui.font_select("##bold_font_idp", &mut self.ui.bold_font) {
            if let Some(font) = self.ui.bold_font {
                unsafe {
                    if let Ok(font_name) = font.name_raw().to_str() {
                        write_config().selected_bold_font_name = Some(font_name.to_string());
                    }
                }
            }
        }
        ui.same_line();
        if ui.button("Reload##idp") {
            load_fonts();
        }
    }

    fn render_max_content_height(&mut self, ui: &Ui<'_>) {
        let max_content_height = read_config().rendering_params.max_content_height;
        let mut new = max_content_height.round() as i32;
        ui.text("Max popup height:");
        ui.input_int("##idp_max_content_height", &mut new)
            .step(50 as _)
            .step_fast(200 as _)
            .build();
        new = new.clamp(320, 1500);
        write_config().rendering_params.max_content_height = new as f32;
    }

    fn render_visibility_options(&mut self, ui: &Ui<'_>) {
        if ui.collapsing_header("Visibility##idp", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
            ui.checkbox(
                "Show general tab##idp",
                &mut write_config().rendering_params.show_general_tab,
            );
            ui.checkbox(
                "Show images tab##idp",
                &mut write_config().rendering_params.show_images_tab,
            );
            ui.checkbox(
                "Show tag bar##idp",
                &mut write_config().rendering_params.show_tag_bar,
            );

            let blacklisted_tabs = &mut write_config().rendering_params.blacklisted_tabs;
            ui.spacing();
            ui.text("Blacklisted tabs:");
            if blacklisted_tabs.is_empty() {
                ui.text_disabled("No tabs");
            } else {
                let mut to_remove_vec = Vec::new();
                if let Some(_t) = ui.begin_table("blacklisted_tabs#idp", 3) {
                    ui.table_next_row();
                    for (i, tab_title) in blacklisted_tabs.iter().enumerate() {
                        ui.table_next_column();
                        ui.text_colored(ERROR_COLOR, "[X]");
                        ui.same_line_with_pos(-10f32);
                        if ui.invisible_button(
                            format!("-##blacklisted_tabs{}", tab_title),
                            [30f32, 30f32],
                        ) {
                            to_remove_vec.push(i);
                        }
                        ui.same_line_with_pos(24f32);
                        ui.text(tab_title);
                    }
                }
                for blacklisted_tab_index in to_remove_vec {
                    blacklisted_tabs.remove(blacklisted_tab_index);
                }
            }
            ui.spacing();
            ui.text("Add to blacklist:");
            ui.same_line();
            ui.text_disabled("(press enter to confirm)");
            ui.input_text("##add_to_blacklist_idp", &mut self.ui.tab_to_blacklist_input)
                .build();
            if ui.is_key_released(Key::Enter) && ui.is_item_focused() {
                let tab = self.ui.tab_to_blacklist_input.to_lowercase();
                if !blacklisted_tabs.contains(&tab) && !self.ui.tab_to_blacklist_input.is_empty() {
                    blacklisted_tabs.push(self.ui.tab_to_blacklist_input.to_lowercase());
                    self.ui.tab_to_blacklist_input = "".to_string();
                }
            }
        }
    }

    fn render_max_content_width(&mut self, ui: &Ui<'_>) {
        let max_content_width = read_config().rendering_params.max_content_width;
        let mut new = max_content_width.round() as i32;
        ui.text("Max popup width:");
        ui.input_int("##idp_max_content_width", &mut new)
            .step(50 as _)
            .step_fast(200 as _)
            .build();
        new = new.clamp(320, 1500);
        write_config().rendering_params.max_content_width = new as f32;
    }
}
