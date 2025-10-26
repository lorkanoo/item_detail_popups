use crate::state::popup::Popup;

#[derive(Clone, Debug, Default)]
pub struct UiContext {
    pub hovered_popup: Option<Popup>,
    pub pinned_popups: Vec<Popup>,
    pub loading_progress: Option<i16>,
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
