use std::collections::BTreeMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::token::Token;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopupData {
    pub item_ids: Option<Vec<u32>>,
    pub item_icon: Option<Token>,
    pub title: String,
    pub description: Vec<Token>,
    pub getting_there: Vec<Token>,
    pub contents: Vec<Token>,
    pub location: Vec<Token>,
    pub notes: Vec<Token>,
    pub walkthrough: Vec<Token>,
    pub rewards: Vec<Token>,
    pub related_achievements: Vec<Token>,
    pub acquisition: Vec<Token>,
    pub teaches_recipe: Vec<Token>,
    pub images: Vec<Token>,
    // tag href, tag name
    pub tags: BTreeMap<String, String>,
    pub cached_date: DateTime<Local>,
    pub href: String,
    pub redirection_href: Option<String>,
}

impl PopupData {
    pub fn is_not_empty(&self) -> bool {
        !self.description.is_empty() 
            || !self.getting_there.is_empty()
            || !self.contents.is_empty()
            || !self.location.is_empty()
            || !self.notes.is_empty()
            || !self.acquisition.is_empty()
            || !self.teaches_recipe.is_empty()
            || !self.walkthrough.is_empty()
            || !self.rewards.is_empty()
            || !self.related_achievements.is_empty()
            || !self.images.is_empty()
            || self.item_ids.is_some()
    }
}

impl Default for PopupData {
    fn default() -> Self {
        Self {
            item_ids: None,
            item_icon: None,
            title: "".to_string(),
            description: vec![],
            getting_there: vec![],
            contents: vec![],
            location: vec![],
            notes: vec![],
            acquisition: vec![],
            teaches_recipe: vec![],
            walkthrough: vec![],
            rewards: vec![],
            related_achievements: vec![],
            images: vec![],
            tags: BTreeMap::new(),
            cached_date: Local::now(),
            href: "".to_string(),
            redirection_href: None,
        }
    }
}

