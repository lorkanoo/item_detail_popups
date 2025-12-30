use crate::configuration::keyboard_layout::KeyboardLayout;
use crate::configuration::DEFAULT_POST_KEY_COMBINATION_DELAY_MS;
use crate::configuration::{read_config, write_config};
use crate::state::context::Context;
use log::debug;
use nexus::imgui::Ui;
use strum::IntoEnumIterator;

const MIN_POST_KEY_COMBINATION_DELAY: i32 = 10;
const MAX_POST_KEY_COMBINATION_DELAY: i32 = 300;

impl Context {
    pub fn render_macro_options(&mut self, ui: &Ui) {
        render_keyboard_layout(ui);
        self.render_post_key_combination_delay(ui);
        ui.checkbox("Use left shift##idp", &mut write_config().use_left_shift);
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
            new = new.clamp(
                MIN_POST_KEY_COMBINATION_DELAY,
                MAX_POST_KEY_COMBINATION_DELAY,
            );
            write_config().post_key_combination_delay_ms = new as u64;
        } else {
            write_config().post_key_combination_delay_ms = DEFAULT_POST_KEY_COMBINATION_DELAY_MS;
        }
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
    write_config().keyboard_layout = layouts
        .get(current_item)
        .expect("Should have expected layout.")
        .clone();
}
