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
}
