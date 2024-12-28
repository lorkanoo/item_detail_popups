use crate::addon::Addon;
use nexus::data_link::mumble::UiState;

pub fn game_has_focus() -> bool {
    if let Some(m) = Addon::lock_context().links.mumble {
        return matches!(
            m.read_ui_state() & UiState::GAME_HAS_FOCUS,
            UiState::GAME_HAS_FOCUS
        );
    }
    false
}

pub fn textbox_has_focus() -> bool {
    if let Some(m) = Addon::lock_context().links.mumble {
        return matches!(
            m.read_ui_state() & UiState::TEXTBOX_HAS_FOCUS,
            UiState::TEXTBOX_HAS_FOCUS
        );
    }
    false
}

pub fn is_in_game() -> bool {
    let mut is_gameplay = false;
    if let Some(nexus) = unsafe { Addon::lock_context().links.nexus() } {
        if nexus.is_gameplay {
            is_gameplay = true;
        }
    }
    is_gameplay
}

pub fn is_on_character_select() -> bool {
    !is_in_game()
}
