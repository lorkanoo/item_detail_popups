use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq)]
pub enum KeyboardLayout {
    QWERTY,
    AZERTY,
    DVORAK,
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        Self::QWERTY
    }
}

impl Display for KeyboardLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardLayout::QWERTY => write!(f, "QWERTY"),
            KeyboardLayout::AZERTY => write!(f, "AZERTY"),
            KeyboardLayout::DVORAK => write!(f, "DVORAK"),
        }
    }
}
