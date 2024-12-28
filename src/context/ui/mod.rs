use crate::context::ui::popup::Popup;

pub mod popup;

#[derive(Clone, Debug, Default)]
pub struct UiContext {
    pub hovered_popup: Option<Popup>,
    pub pinned_popups: Vec<Popup>,
    pub loading_progress: Option<i16>,
}
