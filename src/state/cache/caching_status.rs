use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum CachingStatus {
    #[default]
    Caching,
    Refreshing,
    Cached,
    Failed,
}
