use crate::configuration::config::DEFAULT_POST_KEY_COMBINATION_DELAY_MS;
use crate::configuration::config::{read_config, write_config};
use crate::configuration::keyboard_layout::KeyboardLayout;
use crate::core::utils::ui::UiExtended;
use crate::state::context::Context;
use crate::state::threads::font::load_fonts;
use log::debug;
use nexus::imgui::{Key, TreeNodeFlags, Ui};
use std::time::Duration;
use strum::IntoEnumIterator;

const MAX_REFRESH_HOURS: i32 = 10000;
const MAX_REFRESH_MINUTES: i32 = 59;
const MINIMUM_PRICE_EXPIRATION_SEC: i32 = 15;
const MAX_PRICE_EXPIRATION_SEC: i32 = 300;
const DEFAULT_MAX_CACHED_ELEMENTS: usize = 500;
const ERROR_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

impl Context {
    pub fn render_options(&mut self, ui: &Ui) {
        debug!("[render_options] Started.");

        if let Some(_token) = ui.tab_bar("options#idp") {
            if let Some(_token) = ui.tab_item("Style") {
                self.render_style_options(ui);
            }
            if let Some(_token) = ui.tab_item("Macro") {
                self.render_macro_options(ui);
            }
            if let Some(_token) = ui.tab_item("Cache") {
                self.render_cache_options(ui);
            }
            if let Some(_token) = ui.tab_item("Advanced") {
                self.render_advanced_options(ui);
            }
            if let Some(_token) = ui.tab_item("Help") {
                self.render_help(ui);
            }
        }
    }

    fn render_help(&mut self, ui: &Ui) {
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

    fn render_bold_font_options(&mut self, ui: &Ui) {
        ui.text("Bold font");
        ui.same_line();
        ui.text_disabled(" (place fonts under 'addons/item_detail_popups/fonts')");

        if ui.font_select("##bold_font_idp", &mut self.bold_font) {
            if let Some(font) = self.bold_font {
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

    fn render_style_options(&mut self, ui: &Ui) {
        self.render_bold_font_options(ui);
        ui.checkbox(
            "Use bullet character in list punctuation##idp",
            &mut write_config().rendering_params.use_bullet_list_punctuation,
        );
        self.render_max_content_width(ui);
        self.render_max_content_height(ui);

        render_color_options(ui);
        self.render_visibility_options(ui);
    }

    fn render_macro_options(&mut self, ui: &Ui) {
        render_keyboard_layout(ui);
        self.render_post_key_combination_delay(ui);
        ui.checkbox("Use left shift##idp", &mut write_config().use_left_shift);
    }

    fn render_advanced_options(&mut self, ui: &Ui) {
        ui.checkbox(
            "Wait until all keys are released before opening the popup##idp",
            &mut write_config().wait_until_all_keys_released,
        );
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

    fn render_cache_options(&mut self, ui: &Ui) {
        self.render_max_cached_popup_data_elements(ui);
        self.render_max_popup_data_expiration(ui);
        self.render_price_expiration(ui);
        self.render_max_texture_cache_expiration(ui);
        ui.new_line();
        self.render_cache_used(ui);
        self.render_clear_all_cache(ui);
    }

    fn render_price_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_price_expiration] Started.");
        let price_expiration = read_config()
            .max_price_expiration_duration
            .clone()
            .as_secs();
        if let Ok(mut price_expiration_secs) = i32::try_from(price_expiration) {
            ui.spacing();
            ui.text("Price refresh frequency:");
            ui.input_int("Seconds##itp_prf", &mut price_expiration_secs)
                .build();
            price_expiration_secs =
                price_expiration_secs.clamp(MINIMUM_PRICE_EXPIRATION_SEC, MAX_PRICE_EXPIRATION_SEC);
            write_config().max_price_expiration_duration =
                Duration::from_secs(price_expiration_secs as u64);
        }
    }

    fn render_max_popup_data_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_popup_data_expiration] Started.");

        let max_popup_data_expiration = read_config().max_popup_data_expiration_duration.as_secs();
        let mut hours = (max_popup_data_expiration / 3600) as i32;
        let mut minutes = ((max_popup_data_expiration % 3600) / 60) as i32;

        ui.spacing();
        ui.text("Refresh popups older than:");
        if let Some(_t) = ui.begin_table("popup_cache##idp", 3) {
            ui.table_next_column();
            ui.input_int("Hours", &mut hours).build();
            ui.table_next_column();
            ui.input_int("Minutes", &mut minutes).build();
        }
        hours = hours.clamp(0, MAX_REFRESH_HOURS);
        minutes = minutes.clamp(0, MAX_REFRESH_MINUTES);
        write_config().max_popup_data_expiration_duration =
            Duration::from_secs(hours as u64 * 3600 + minutes as u64 * 60);
    }

    fn render_max_texture_cache_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_texture_cache_expiration] Started.");

        let max_texture_cache_expiration = read_config().max_texture_expiration_duration.as_secs();
        let mut hours = (max_texture_cache_expiration / 3600) as i32;
        let mut minutes = ((max_texture_cache_expiration % 3600) / 60) as i32;

        ui.spacing();
        ui.text("Refresh images older than:");
        if let Some(_t) = ui.begin_table("texture_cache##idp", 3) {
            ui.table_next_column();
            ui.input_int("Hours", &mut hours).build();
            ui.table_next_column();
            ui.input_int("Minutes", &mut minutes).build();
        }
        hours = hours.clamp(0, MAX_REFRESH_HOURS);
        minutes = minutes.clamp(0, MAX_REFRESH_MINUTES);
        write_config().max_texture_expiration_duration =
            Duration::from_secs(hours as u64 * 3600 + minutes as u64 * 60);
    }

    fn render_max_cached_popup_data_elements(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_cached_popup_data_elements] Started.");
        let max_popup_data_elements = read_config().max_popup_data_elements;
        if let Ok(mut new) = i32::try_from(max_popup_data_elements) {
            ui.text("Max cached popups:");
            ui.input_int("##idp_mcp", &mut new)
                .step(1 as _)
                .step_fast(10 as _)
                .build();
            new = new.clamp(0, 2000);
            write_config().max_popup_data_elements = new as usize;
        } else {
            write_config().max_popup_data_elements = DEFAULT_MAX_CACHED_ELEMENTS;
        }
    }

    fn render_post_key_combination_delay(&mut self, ui: &Ui<'_>) {
        debug!("[render_post_key_combination_delay] Started.");
        let post_key_combination_delay_ms = read_config().post_key_combination_delay_ms;
        if let Ok(mut new) = i32::try_from(post_key_combination_delay_ms) {
            ui.text("Macro delay (ms)");
            ui.same_line();
            ui.text_disabled("(too low values will cause popups not opening)");
            ui.input_int("##idp_pcd", &mut new)
                .step(10 as _)
                .step_fast(100 as _)
                .build();
            new = new.clamp(10, 300);
            write_config().post_key_combination_delay_ms = new as u64;
        } else {
            write_config().post_key_combination_delay_ms = DEFAULT_POST_KEY_COMBINATION_DELAY_MS;
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

    fn render_cache_used(&mut self, ui: &Ui<'_>) {
        debug!("[render_cache_used] Started.");
        let mut cache_used = 0.00;
        let cache_elements = read_config().max_popup_data_elements;
        if cache_elements > 0 {
            cache_used = self.cache.popup_data_map.len() as f32 / cache_elements as f32 * 100.0;
        }
        ui.text(format!("{:.2}% cache used", cache_used));
    }

    fn render_clear_all_cache(&mut self, ui: &Ui<'_>) {
        debug!("[render_clear_all_cache] Started.");
        if ui.button("Clear all Cache") {
            self.cache.evict();
        }
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
            ui.input_text("##add_to_blacklist_idp", &mut self.tab_to_blacklist_input)
                .build();
            if ui.is_key_released(Key::Enter) && ui.is_item_focused() {
                let tab = self.tab_to_blacklist_input.to_lowercase();
                if !blacklisted_tabs.contains(&tab) && !self.tab_to_blacklist_input.is_empty() {
                    blacklisted_tabs.push(self.tab_to_blacklist_input.to_lowercase());
                    self.tab_to_blacklist_input = "".to_string();
                }
            }
        }
    }
}

fn render_color_options(ui: &Ui<'_>) {
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

fn render_keyboard_layout(ui: &Ui<'_>) {
    let layouts: Vec<KeyboardLayout> = KeyboardLayout::iter().collect();
    let mut current_item = layouts
        .iter()
        .position(|v| *v == read_config().keyboard_layout)
        .unwrap();

    ui.text("Keyboard layout:");
    ui.combo("##kl_idp", &mut current_item, &layouts, |selected_layout| {
        format!("{}", selected_layout).into()
    });
    write_config().keyboard_layout = layouts.get(current_item).expect("Expected layout").clone();
}
