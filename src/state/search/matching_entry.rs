#[derive(Clone, Debug)]
pub struct MatchingSearchEntry {
    pub text: String,
    pub href: String,
}

impl MatchingSearchEntry {
    pub fn new(text: String, href: String) -> Self {
        Self { text, href }
    }
}