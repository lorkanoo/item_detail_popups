use log::{error, warn};

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
