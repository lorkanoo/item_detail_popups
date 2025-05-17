use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagParams {
    pub href: String,
    pub text: String,
    pub title: String,
}

