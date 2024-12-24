pub mod extended;

pub const SUCCESS_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const LINK_COLOR: [f32; 4] = [0.2, 0.4, 0.8, 1.0];

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum UiAction {
    MoveDown(usize),
    MoveUp(usize),
    Delete(usize),
    Clone(usize),
    Close,
    Pin,
    Open(String, String),
}

pub trait UiElement {
    fn rename(&mut self, _new_name: String) {}
    fn name(&self) -> &String;
}

pub fn process_ui_actions_for_vec<T: UiElement + Clone>(
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
            _ => return true,
        }
        false
    });
}
