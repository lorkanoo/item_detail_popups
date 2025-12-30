use crate::configuration::{read_config, write_config};
use crate::state::context::Context;
use crate::utils::time::HourAndMinute;
use log::debug;
use nexus::imgui::Ui;
use std::time::Duration;

const MAX_REFRESH_HOURS: i32 = 10000;
const MAX_REFRESH_MINUTES: i32 = 59;
const MINIMUM_PRICE_EXPIRATION_SEC: i32 = 15;
const MAX_PRICE_EXPIRATION_SEC: i32 = 300;
const DEFAULT_MAX_CACHED_ELEMENTS: usize = 500;

impl Context {
    pub fn render_cache_options(&mut self, ui: &Ui) {
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

        let mut max_expiration: HourAndMinute =
            read_config().max_popup_data_expiration_duration.into();

        ui.spacing();
        ui.text("Refresh popups older than:");
        if let Some(_t) = ui.begin_table("popup_cache##idp", 3) {
            ui.table_next_column();
            ui.input_int("Hours", &mut max_expiration.hour).build();
            ui.table_next_column();
            ui.input_int("Minutes", &mut max_expiration.minute).build();
        }
        max_expiration.hour = max_expiration.hour.clamp(0, MAX_REFRESH_HOURS);
        max_expiration.minute = max_expiration.minute.clamp(0, MAX_REFRESH_MINUTES);
        write_config().max_popup_data_expiration_duration = max_expiration.into();
    }

    fn render_max_texture_cache_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_texture_cache_expiration] Started.");

        let mut max_expiration: HourAndMinute =
            read_config().max_texture_expiration_duration.into();

        ui.spacing();
        ui.text("Refresh images older than:");
        if let Some(_t) = ui.begin_table("texture_cache##idp", 3) {
            ui.table_next_column();
            ui.input_int("Hours", &mut max_expiration.hour).build();
            ui.table_next_column();
            ui.input_int("Minutes", &mut max_expiration.minute).build();
        }
        max_expiration.hour = max_expiration.hour.clamp(0, MAX_REFRESH_HOURS);
        max_expiration.minute = max_expiration.minute.clamp(0, MAX_REFRESH_MINUTES);
        write_config().max_texture_expiration_duration = max_expiration.into();
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
}
