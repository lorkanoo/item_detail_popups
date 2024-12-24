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

                    trigger_key_combination(&KeyCombination::shift_click());
                    if !textbox_has_focus() {
                        debug!("Could not process - could not link item to chat");
                        Addon::lock().context.ui.loading = None;
                        return;
                    }

                    cut_all_and_close_chat();
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
    let mut retries = 5;
    loop {
        if retries == 0 {
            break;
        }
        retries -= 1;
        let clipboard_text = Addon::lock().context.clipboard.get_text();
        let clipboard_retrieved = match clipboard_text {
            Ok(clipboard_text) => {
                let item_name = extract_item_name(&clipboard_text);
                match item_name {
                    Ok(item_name) => {
                        Addon::lock().context.ui.hovered_popup =
                            Some(prepare_item_popup(&item_name));
                    }
                    Err(e) => debug!("{}", e),
                }
                true
            }
            Err(e) => {
                warn!("Couldn't get clipboard text: {}", e);
                false
            }
        };
        if clipboard_retrieved {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
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
    loop {
        let clipboard_text = Addon::lock().context.clipboard.get_text();
        if !clipboard_text.is_ok() {
            break;
        }
        if let Err(_) = Addon::lock().context.clipboard.clear() {
            debug!("Couldn't clear clipboard content.");
        }
        thread::sleep(Duration::from_millis(20));
    }
}
