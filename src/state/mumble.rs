use crate::state::context::write_context;
use nexus::data_link::mumble::UiState;

pub fn textbox_has_focus() -> bool {
    if let Some(m) = write_context().links.mumble {
        return matches!(
            m.read_ui_state() & UiState::TEXTBOX_HAS_FOCUS,
            UiState::TEXTBOX_HAS_FOCUS
        );
    }
    false
}
