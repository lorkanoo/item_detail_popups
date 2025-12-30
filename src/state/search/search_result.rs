use crate::state::popup::Popup;
use crate::state::search::matching_entry::MatchingSearchEntry;

#[derive(Clone, Debug)]
pub enum SearchResult {
    SingleMatch(Popup),
    MultipleMatches(Vec<MatchingSearchEntry>),
}

