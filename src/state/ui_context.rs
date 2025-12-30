use crate::state::font::Font;
use crate::state::popup::Popup;
use crate::state::search::search_result::SearchResult;

#[derive(Clone, Debug, Default)]
pub struct UiContext {
    pub hovered_popup: Option<Popup>,
    pub pinned_popups: Vec<Popup>,
    pub loading_progress: Option<i16>,
    pub search_result: Option<SearchResult>,
    pub search_popup_input: String,
    pub gw2_api_key_input: String,
    pub should_open_search_prompt: bool,
    pub should_open_search_result: bool,
    pub search_opened: bool,
    pub search_position: [f32; 2],
    pub bold_font: Option<Font>,
    pub tab_to_blacklist_input: String,
}

impl UiContext {
    pub fn close_all_popups(&mut self) {
        self.pinned_popups
            .iter_mut()
            .for_each(|p| p.state.opened = false);
        if let Some(p) = &mut self.hovered_popup {
            p.state.opened = false;
        }
    }
}
