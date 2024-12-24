use crate::context::ui::popup::Popup;

pub mod popup;

#[derive(Clone, Debug)]
pub struct UiContext {
    pub hovered_popup: Option<Popup>,
    pub pinned_popups: Vec<Popup>,
    pub loading: Option<i16>,
}

#[derive(Clone, Debug)]
pub struct Errors {}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            hovered_popup: None,
            pinned_popups: vec![],
            loading: None,
        }
    }
}
