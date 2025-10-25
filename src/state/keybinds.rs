use crate::api::gw2_wiki::prepare_item_popup_with_quantity;
use crate::core::threads::lock_threads;
use crate::core::utils::item_tag_parser::extract_item_details;
use crate::core::utils::key_combination::{trigger_key_combination, KeyCombination};
use crate::state::context::{read_context, write_context};
use crate::state::mumble::textbox_has_focus;
use log::{debug, error};
use nexus::keybind::register_keybind_with_string;
use nexus::keybind_handler;
use std::thread;
use std::time::Duration;

const MAX_COPY_ATTEMPTS: u32 = 2;
const MAX_CLIPBOARD_CLEAR_ATTEMPTS: u32 = 30;

pub fn register_show_popup_keybind() {
    let keybind_handler = keybind_handler!(|_id, is_release| {
        debug!("[register_show_popup_keybind]");
        if is_release {
            lock_threads().push(thread::spawn(|| {
                debug!("[register_show_popup_keybind thread start]");
                write_context().ui.loading_progress = Some(1);
                write_context().ui.hovered_popup = None;
                await_cleared_clipboard();

                let mut copy_attempts = 0;
                let last_clipboard_text = read_context().last_clipboard_text.clone();

                while copy_attempts < MAX_COPY_ATTEMPTS {
                    debug!("[keybind_handler] Copy attempt: {}", copy_attempts);
                    trigger_key_combination(&KeyCombination::shift_click());
                    if !textbox_has_focus() {
                        debug!("Could not process - could not link item to chat");
                        write_context().ui.loading_progress = None;
                        return;
                    }

                    cut_all_and_close_chat();
                    thread::sleep(Duration::from_millis(30));

                    if has_clipboard_changed(&last_clipboard_text) {
                        break;
                    }

                    copy_attempts += 1;
                    thread::sleep(Duration::from_millis(30));
                }
                process_clipboard_text();

                if textbox_has_focus() {
                    debug!("[keybind_handler] Textbox has focus - clearing and closing");
                    clear_all_and_close_chat();
                }
                await_cleared_clipboard();
                write_context().ui.loading_progress = None;
                debug!("[register_show_popup_keybind thread end]");
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

pub fn register_open_search_keybind() {
    let keybind_handler = keybind_handler!(|_id, is_release| {
        debug!("[register_open_search_keybind]");
        if is_release {
            write_context().should_open_search = true;
        }
    });
    register_keybind_with_string("Open search", keybind_handler, "CTRL+SHIFT+F").revert_on_unload();
}

fn process_clipboard_text() {
    let clipboard_text = write_context().clipboard.get_text();
    match clipboard_text {
        Ok(clipboard_text) => {
            debug!("[process_clipboard_text] text = {}", clipboard_text);
            write_context().last_clipboard_text = Some(clipboard_text.clone());
            let item_details = extract_item_details(&clipboard_text);
            match item_details {
                Ok(item_details) => {
                    write_context().ui.hovered_popup = Some(prepare_item_popup_with_quantity(
                        &item_details.name,
                        &item_details.quantity,
                    ));
                }
                Err(e) => error!("{}", e),
            }
        }
        Err(e) => {
            error!("Couldn't get clipboard text: {}", e);
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
        debug!("[await_cleared_clipboard] remaining retries: {}", retries);
        let clipboard_text = write_context().clipboard.get_text();
        if clipboard_text.is_err() {
            debug!("[await_cleared_clipboard] error: {clipboard_text:?}");
            break;
        }
        if write_context().clipboard.clear().is_err() {
            debug!("[await_cleared_clipboard] Couldn't clear clipboard content.");
        }
        thread::sleep(Duration::from_millis(20));
        retries -= 1;
    }
    debug!("[await_cleared_clipboard] end");
}

fn has_clipboard_changed(last_clipboard_text: &Option<String>) -> bool {
    if let Ok(current_clipboard) = write_context().clipboard.get_text() {
        if let Some(ref last_text) = last_clipboard_text {
            return *last_text != current_clipboard;
        }
    }
    false
}
