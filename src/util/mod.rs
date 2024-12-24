pub mod key_combination;

use crate::addon::Addon;
use nexus::data_link::mumble::UiState;
use singularize::singularize_item_name;

pub fn game_has_focus() -> bool {
    if let Some(m) = Addon::lock().context.links.mumble {
        return matches!(
            m.read_ui_state() & UiState::GAME_HAS_FOCUS,
            UiState::GAME_HAS_FOCUS
        );
    }
    false
}

pub fn textbox_has_focus() -> bool {
    if let Some(m) = Addon::lock().context.links.mumble {
        return matches!(
            m.read_ui_state() & UiState::TEXTBOX_HAS_FOCUS,
            UiState::TEXTBOX_HAS_FOCUS
        );
    }
    false
}

pub fn is_in_game() -> bool {
    let mut is_gameplay = false;
    if let Some(nexus) = unsafe { Addon::lock().context.links.nexus() } {
        if nexus.is_gameplay {
            is_gameplay = true;
        }
    }
    is_gameplay
}

pub fn is_on_character_select() -> bool {
    !is_in_game()
}

pub fn true_if_1() -> fn(&String) -> bool {
    |value| value == "1"
}

pub fn extract_item_name(chat_message: &str) -> Result<String, &'static str> {
    let start = chat_message.find("[");
    let end = chat_message.find("]");
    match (start, end) {
        (Some(start), Some(end)) => {
            if chat_message.len() < 3 || start > end {
                return Err("Could not extract item name: invalid item tag");
            }
            let mut item_tag = chat_message[start + 1..end].to_string();
            item_tag = item_tag.replace("Recipe: ", "");
            Ok(singularize_item_name(&item_tag))
        }
        _ => Err("Could not extract item name: invalid chat message"),
    }
}
