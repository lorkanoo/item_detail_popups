use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum CachingStatus {
    #[default]
    InProgress,
    Cached,
    Failed,
}
