use crate::{addon::Addon, context::Context};
use log::debug;
use nexus::imgui::Ui;
use std::time::Duration;

const MAX_REFRESH_HOURS: i32 = 10000;
const MAX_REFRESH_MINUTES: i32 = 59;
const MINIMUM_PRICE_EXPIRATION_SEC: i32 = 10;
const MAX_PRICE_EXPIRATION_SEC: i32 = 300;
const DEFAULT_MAX_CACHED_ELEMENTS: usize = 500;

impl Context {
    pub fn render_options(&mut self, ui: &Ui) {
        debug!("[render_options] Started.");
        self.render_max_cached_popup_data_elements(ui);
        self.render_max_popup_data_cache_expiration(ui);
        self.render_price_expiration(ui);
        ui.spacing();
        self.render_cache_used(ui);
        self.render_clear_all_cache(ui);
    }

    fn render_price_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_price_expiration] Started.");
        let price_expiration = Addon::lock_config()
            .price_expiration_duration
            .clone()
            .as_secs();
        if let Ok(mut price_expiration_secs) = i32::try_from(price_expiration) {
            ui.spacing();
            ui.input_int(
                "Price refresh frequency (seconds)",
                &mut price_expiration_secs,
            )
            .build();
            price_expiration_secs =
                price_expiration_secs.clamp(MINIMUM_PRICE_EXPIRATION_SEC, MAX_PRICE_EXPIRATION_SEC);
            Addon::lock_config().price_expiration_duration =
                Duration::from_secs(price_expiration_secs as u64);
        }
    }

    fn render_max_popup_data_cache_expiration(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_popup_data_cache_expiration] Started.");

        let max_popup_data_cache_expiration = Addon::lock_config()
            .max_popup_data_cache_expiration_duration
            .as_secs();
        let mut hours = (max_popup_data_cache_expiration / 3600) as i32;
        let mut minutes = ((max_popup_data_cache_expiration % 3600) / 60) as i32;

        ui.spacing();
        ui.text("Refresh popups older than:");
        ui.input_int("Hours", &mut hours).build();
        ui.input_int("Minutes", &mut minutes).build();
        hours = hours.clamp(0, MAX_REFRESH_HOURS);
        minutes = minutes.clamp(0, MAX_REFRESH_MINUTES);
        Addon::lock_config().max_popup_data_cache_expiration_duration =
            Duration::from_secs(hours as u64 * 3600 + minutes as u64 * 60);
    }

    fn render_max_cached_popup_data_elements(&mut self, ui: &Ui<'_>) {
        debug!("[render_max_cached_popup_data_elements] Started.");
        let max_popup_data_cache_elements = Addon::lock_config().max_popup_data_cache_elements;
        if let Ok(mut new) = i32::try_from(max_popup_data_cache_elements) {
            ui.input_int("Max cached popups", &mut new)
                .step(1 as _)
                .step_fast(10 as _)
                .build();
            new = new.clamp(0, 2000);
            Addon::lock_config().max_popup_data_cache_elements = new as usize;
        } else {
            Addon::lock_config().max_popup_data_cache_elements = DEFAULT_MAX_CACHED_ELEMENTS;
        }
    }

    fn render_cache_used(&mut self, ui: &Ui<'_>) {
        debug!("[render_cache_used] Started.");
        let mut cache_used = 0.00;
        let cache_elements = Addon::lock_config().max_popup_data_cache_elements;
        if cache_elements > 0 {
            cache_used =
                Addon::lock_cache().popup_data_map.len() as f32 / cache_elements as f32 * 100.0;
        }
        ui.text(format!("{:.2}% cache used", cache_used));
    }

    fn render_clear_all_cache(&mut self, ui: &Ui<'_>) {
        debug!("[render_clear_all_cache] Started.");
        if ui.button("Clear all Cache") {
            Addon::lock_cache().evict();
        }
    }
}
