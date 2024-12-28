use crate::addon::Addon;
use crate::api::gw2_wiki::prepare_item_popup;
use crate::util::key_combination::{trigger_key_combination, KeyCombination};
use crate::util::{extract_item_name, textbox_has_focus};
use log::{debug, warn};
use nexus::keybind::{register_keybind_with_struct, Keybind};
use nexus::keybind_handler;
use std::thread;
use std::time::Duration;

impl Addon {
    pub fn register_show_popup_keybind() {
        let keybind_handler = keybind_handler!(|_id, is_release| {
            if is_release {
                Addon::threads().push(thread::spawn(|| {
                    Addon::lock().context.ui.loading = Some(1);
                    Addon::lock().context.ui.hovered_popup = None;
                    await_cleared_clipboard();

                    let mut attempts = 0;
                    let last_clipboard_text = Addon::lock().context.last_clipboard_text.clone();

                    while attempts < 2 {
                        trigger_key_combination(&KeyCombination::shift_click());
                        if !textbox_has_focus() {
                            debug!("Could not process - could not link item to chat");
                            Addon::lock().context.ui.loading = None;
                            return;
                        }

                        cut_all_and_close_chat();
                        thread::sleep(Duration::from_millis(50));
                        // Check if clipboard changed
                        if let Ok(current_clipboard) = Addon::lock().context.clipboard.get_text() {
                            if let Some(ref last_clipboard_text) = last_clipboard_text {
                                if *last_clipboard_text != current_clipboard {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        attempts += 1;
                        thread::sleep(Duration::from_millis(50));
                    }
                    process_clipboard_text();

                    if textbox_has_focus() {
                        debug!("Textbox has focus - clearing and closing");
                        clear_all_and_close_chat();
                    }
                    await_cleared_clipboard();
                    Addon::lock().context.ui.loading = None;
                }));
            };
        });
        register_keybind_with_struct(
            "Show details of a hovered item",
            keybind_handler,
            Keybind {
                key: 42,
                ctrl: true,
                shift: true,
                alt: false,
            },
        )
        .revert_on_unload();
    }
}

fn process_clipboard_text() {
    let clipboard_text = Addon::lock().context.clipboard.get_text();
    match clipboard_text {
        Ok(clipboard_text) => {
            Addon::lock().context.last_clipboard_text = Some(clipboard_text.clone());
            let item_name = extract_item_name(&clipboard_text);
            match item_name {
                Ok(item_name) => {
                    Addon::lock().context.ui.hovered_popup = Some(prepare_item_popup(&item_name));
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
    let mut retries = 30;
    while retries > 0 {
        let clipboard_text = Addon::lock().context.clipboard.get_text();
        if clipboard_text.is_err() {
            break;
        }
        if Addon::lock().context.clipboard.clear().is_err() {
            debug!("Couldn't clear clipboard content.");
        }
        thread::sleep(Duration::from_millis(20));
        retries -= 1;
    }
}
