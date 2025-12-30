use crate::configuration::search::normalize::Normalize;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistory<T: PartialEq + Normalize<T>> {
    data: VecDeque<T>,
    max_size: usize,
}

impl<T: PartialEq + Normalize<T>> SearchHistory<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: VecDeque::new(),
            max_size,
        }
    }

    pub fn push(&mut self, item: T) {
        let normalized = item.normalize();
        if let Some(pos) = self.data.iter().position(|x| x == &normalized) {
            self.data.remove(pos);
        }
        while self.is_full() {
            self.data.pop_back();
        }

        self.data.push_front(normalized);
    }

    fn is_full(&self) -> bool {
        self.data.len() >= self.max_size
    }
}

impl SearchHistory<String> {
    pub fn find_containing(&self, needle: &str, max_results: usize) -> Vec<&String> {
        let normalized = needle.to_string().normalize();
        self.data
            .iter()
            .filter(|x| x.contains(normalized.as_str()))
            .take(max_results)
            .collect()
    }
}
