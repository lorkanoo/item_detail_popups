use crate::addon::Addon;
use nexus::imgui::Ui;

impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        let mut cache_used = 0.00;
        if self.config.max_popup_cache_size > 0 {
            cache_used = Addon::cache().popups.len() as f32
                / self.config.max_popup_cache_size as f32
                * 100.0;
        }
        if let Ok(mut new) = i32::try_from(self.config.max_popup_cache_size) {
            ui.input_int("Max cached popups", &mut new)
                .step(1 as _)
                .step_fast(10 as _)
                .build();
            new = new.clamp(0, 2000);
            self.config.max_popup_cache_size = new as usize;
        } else {
            self.config.max_popup_cache_size = 500;
        }
        if let (Ok(mut hours), Ok(mut minutes)) = (
            i32::try_from(self.config.max_popup_cache_expiration.0),
            i32::try_from(self.config.max_popup_cache_expiration.1),
        ) {
            ui.spacing();
            ui.text("Refresh popups older than:");
            ui.input_int("Hours", &mut hours).build();
            ui.input_int("Minutes", &mut minutes).build();
            if hours < 0 {
                hours = 0;
            }
            if minutes < 0 {
                minutes = 0;
            }
            if hours > 10000 {
                hours = 10000;
            }
            if minutes > 59 {
                minutes = 59;
            }
            self.config.max_popup_cache_expiration.0 = hours as i64;
            self.config.max_popup_cache_expiration.1 = minutes as i64;
        }
        ui.spacing();
        ui.text(format!("{:.2}% cache used", cache_used));
    }
}
