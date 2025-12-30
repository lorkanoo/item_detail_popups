use serde::{Deserialize, Serialize};
use crate::configuration::search::search_history::SearchHistory;

pub const DEFAULT_SEARCH_HISTORY_SIZE: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub search_history: SearchHistory<String>,
    pub max_search_results: usize,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            search_history: SearchHistory::new(DEFAULT_SEARCH_HISTORY_SIZE),
            max_search_results: 5,
        }
    }
}
