use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Style {
    Normal,
    Bold,
    Disabled,
}

