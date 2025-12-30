use crate::state::context::read_context;
use popup_data::PopupData;
use popup_state::PopupState;

pub mod dimensions;
pub mod popup_data;
pub mod popup_state;
pub mod style;
pub mod table_params;
pub mod tag_params;
pub mod token;

#[derive(Clone, Debug)]
pub struct Popup {
    pub data: PopupData,
    pub state: PopupState,
}

impl Popup {
    pub fn new(data: PopupData) -> Self {
        Self {
            state: PopupState::new(),
            data,
        }
    }

    pub fn new_with(href: &str, title: String, item_quantity: &usize) -> Self {
        let mut data = PopupData {
            title: title.clone(),
            href: href.to_owned(),
            ..PopupData::default()
        };
        if let Some(item_names) = read_context().cache.item_names.value() {
            data.item_ids = item_names.get(&title).cloned();
        }

        Self {
            data,
            state: PopupState::new_with_quantity(*item_quantity),
        }
    }
}
