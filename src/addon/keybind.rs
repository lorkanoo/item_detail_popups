use crate::addon::Addon;
use crate::api::gw2_wiki::prepare_item_popup;
use crate::util::extract_item_name;
use crate::util::key_combination::{trigger_key_combination, KeyCombination};
use crate::util::ui_state::textbox_has_focus;
use log::{debug, warn};
use nexus::keybind::register_keybind_with_string;
use nexus::keybind_handler;
use std::thread;
use std::time::Duration;

const MAX_COPY_ATTEMPTS: u32 = 2;
const MAX_CLIPBOARD_CLEAR_ATTEMPTS: u32 = 30;

impl Addon {
    pub fn register_show_popup_keybind() {
        let keybind_handler = keybind_handler!(|_id, is_release| {
            if is_release {
                Addon::lock_threads().push(thread::spawn(|| {
                    Addon::lock_context().ui.loading_progress = Some(1);
                    Addon::lock_context().ui.hovered_popup = None;
                    await_cleared_clipboard();

                    let mut copy_attempts = 0;
                    let last_clipboard_text = Addon::lock_context().last_clipboard_text.clone();

                    while copy_attempts < MAX_COPY_ATTEMPTS {
                        trigger_key_combination(&KeyCombination::shift_click());
                        if !textbox_has_focus() {
                            debug!("Could not process - could not link item to chat");
                            Addon::lock_context().ui.loading_progress = None;
                            return;
                        }

                        cut_all_and_close_chat();
                        thread::sleep(Duration::from_millis(20));

                        if has_clipboard_changed(&last_clipboard_text) {
                            break;
                        }

                        copy_attempts += 1;
                        thread::sleep(Duration::from_millis(20));
                    }
                    process_clipboard_text();

                    if textbox_has_focus() {
                        debug!("Textbox has focus - clearing and closing");
                        clear_all_and_close_chat();
                    }
                    await_cleared_clipboard();
                    Addon::lock_context().ui.loading_progress = None;
                }));
            };
        });
        register_keybind_with_string(
            "Show details of a hovered item",
            keybind_handler,
            "CTRL+SHIFT+X",
        )
        .revert_on_unload();
    }
}

fn process_clipboard_text() {
    let clipboard_text = Addon::lock_context().clipboard.get_text();
    match clipboard_text {
        Ok(clipboard_text) => {
            Addon::lock_context().last_clipboard_text = Some(clipboard_text.clone());
            let item_name = extract_item_name(&clipboard_text);
            match item_name {
                Ok(item_name) => {
                    Addon::lock_context().ui.hovered_popup = Some(prepare_item_popup(&item_name));
                }
                Err(e) => debug!("{}", e),
            }
        }
        Err(e) => {
            warn!("Couldn't get clipboard text: {}", e);
        }
    };
}
fn cut_all_and_close_chat() {
    trigger_key_combination(&KeyCombination::select_all());
    trigger_key_combination(&KeyCombination::cut());
    trigger_key_combination(&KeyCombination::enter());
}

fn clear_all_and_close_chat() {
    trigger_key_combination(&KeyCombination::select_all());
    trigger_key_combination(&KeyCombination::backspace());
    trigger_key_combination(&KeyCombination::enter());
}

fn await_cleared_clipboard() {
    let mut retries = MAX_CLIPBOARD_CLEAR_ATTEMPTS;
    while retries > 0 {
        let clipboard_text = Addon::lock_context().clipboard.get_text();
        if clipboard_text.is_err() {
            break;
        }
        if Addon::lock_context().clipboard.clear().is_err() {
            debug!("Couldn't clear clipboard content.");
        }
        thread::sleep(Duration::from_millis(20));
        retries -= 1;
    }
}

fn has_clipboard_changed(last_clipboard_text: &Option<String>) -> bool {
    if let Ok(current_clipboard) = Addon::lock_context().clipboard.get_text() {
        if let Some(ref last_text) = last_clipboard_text {
            return *last_text != current_clipboard;
        }
    }
    false
}
