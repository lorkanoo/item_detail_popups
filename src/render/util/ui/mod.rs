pub mod extended;

pub const HIGHLIGHT_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const GOLD_COLOR: [f32; 4] = [1.0, 0.843, 0.0, 1.0];
pub const SILVER_COLOR: [f32; 4] = [0.75, 0.75, 0.75, 1.0];
pub const COPPER_COLOR: [f32; 4] = [0.72, 0.45, 0.20, 1.0];

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
