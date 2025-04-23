use log::debug;

use crate::{addon::Addon, thread::refresh_popup_thread};

pub mod extended;

pub const SUCCESS_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const LINK_COLOR: [f32; 4] = [0.2, 0.4, 0.8, 1.0];
pub const GOLD_COLOR: [f32; 4] = [1.0, 0.843, 0.0, 1.0];
pub const SILVER_COLOR: [f32; 4] = [0.75, 0.75, 0.75, 1.0];
pub const COPPER_COLOR: [f32; 4] = [0.72, 0.45, 0.20, 1.0];

#[derive(Clone, Debug)]
pub enum UiAction {
    MoveDown(usize),
    MoveUp(usize),
    Delete(usize),
    Clone(usize),
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

pub trait Linkable {
    fn href(&self) -> &String;
    fn redirection_href(&self) -> &Option<String>;
}

pub trait UiElement {
    fn rename(&mut self, _new_name: String) {}
    fn name(&self) -> &String;
    fn id(&self) -> &u64;
    fn pos(&self) -> &Option<[f32; 2]>;
}

pub fn process_ui_actions_for_vec<T: UiElement + Clone + Linkable>(
    vec: &mut Vec<T>,
    ui_actions: &mut Vec<UiAction>,
) {
    ui_actions.retain(|action| {
        match action {
            UiAction::MoveDown(i) => vec.swap(*i, *i + 1),
            UiAction::MoveUp(i) => vec.swap(*i, *i - 1),
            UiAction::Delete(i) => {
                vec.remove(*i);
            }
            UiAction::Clone(i) => {
                if let Some(t) = vec.get(*i) {
                    let mut new_t = t.clone();
                    new_t.rename(format!("{} (1)", new_t.name()));
                    vec.insert(0, new_t);
                }
            }
            UiAction::Refresh(i) => {
                if let Some(t) = vec.get(*i) {
                    debug!(
                        "[process_ui_actions_for_vec] Refreshing popup with href: {}",
                        t.href()
                    );
                    Addon::lock_cache().popup_data_map.swap_remove(t.href());
                    if let Some(href) = t.redirection_href() {
                        Addon::lock_cache().popup_data_map.swap_remove(href);
                    }
                    refresh_popup_thread(*t.id(), t.name().clone(), *t.pos());
                    vec.remove(*i);
                }
            }
            _ => return true,
        }
        false
    });
}
