use crate::utils::serde::yes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationParams {
    #[serde(default = "yes")]
    pub show_item_name_copy_tip: bool,

    #[serde(default = "yes")]
    pub show_close_all_tip: bool,
}

impl Default for NotificationParams {
    fn default() -> Self {
        Self {
            show_item_name_copy_tip: yes(),
            show_close_all_tip: yes(),
        }
    }
}
