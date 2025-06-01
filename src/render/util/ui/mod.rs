pub mod extended;

pub const HIGHLIGHT_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

#[derive(Clone, Debug)]
pub enum UiAction {
    Delete(usize),
    Refresh(usize),
    Close,
    Pin,
    Open(UiLink),
}

#[derive(Clone, Debug)]
pub struct UiLink {
    pub title: String,
    pub href: String,
}
