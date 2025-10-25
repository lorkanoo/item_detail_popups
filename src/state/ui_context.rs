use crate::state::popup::Popup;

#[derive(Clone, Debug, Default)]
pub struct UiContext {
    pub hovered_popup: Option<Popup>,
    pub pinned_popups: Vec<Popup>,
    pub loading_progress: Option<i16>,
}
