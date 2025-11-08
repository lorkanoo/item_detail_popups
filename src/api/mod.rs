use log::{error, warn};

pub mod gw2_tp;
pub mod gw2_wiki;
pub mod gw2;

pub enum ApiError {
    Internal(String),
    Unexpected(String),
}

impl ApiError {
    pub fn log(&self) {
        match self {
            ApiError::Internal(e) => error!("Internal error: {e}"),
            ApiError::Unexpected(e) => warn!("Unexpected error: {e}"),
        }
    }
}