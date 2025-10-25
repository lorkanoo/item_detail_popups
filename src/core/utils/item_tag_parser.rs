use lazy_static::lazy_static;
use regex::{Match, Regex};
use singularize::singularize;

const ITEM_NAME_MINIMUM_CHARACTERS: usize = 3;
const TAG_START_CHAR: char = '[';
const TAG_END_CHAR: char = ']';
const RECIPE_PREFIX: &str = "Recipe: ";

lazy_static! {
    static ref REMOVE_ITEM_COUNT_REGEX: Regex = Regex::new(r"^\d+ ").unwrap();
}

pub struct ItemDetails {
    pub name: String,
    pub quantity: usize,
}

impl ItemDetails {
    pub fn new(name: String, quantity: usize) -> ItemDetails {
        ItemDetails { name, quantity }
    }
}

pub fn extract_item_details(chat_message: &str) -> Result<ItemDetails, &'static str> {
    if let (Some(tag_start), Some(tag_end)) = (
        chat_message.find(TAG_START_CHAR),
        chat_message.find(TAG_END_CHAR),
    ) {
        if chat_message.len() < ITEM_NAME_MINIMUM_CHARACTERS
            || does_not_contain_tag(tag_start, tag_end)
        {
            return Err("Could not extract item name: invalid item tag");
        }
        let mut item_name = chat_message[tag_start + 1..tag_end].to_string();
        let quantity = item_name
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(1);

        remove_recipe_item_prefix(&mut item_name);
        Ok(ItemDetails::new(
            singularize_item_name(&item_name),
            quantity,
        ))
    } else {
        Err("Could not extract item name: invalid chat message")
    }
}

fn remove_recipe_item_prefix(item_name: &mut String) {
    *item_name = item_name.replace(RECIPE_PREFIX, "")
}

fn does_not_contain_tag(tag_start: usize, tag_end: usize) -> bool {
    tag_start > tag_end
}

fn singularize_item_name(item_tag: &str) -> String {
    let mut result = item_tag.to_owned();
    let matched_item_quantity: Vec<Match> = REMOVE_ITEM_COUNT_REGEX.find_iter(item_tag).collect();
    let has_quantity = !matched_item_quantity.is_empty();
    for i in matched_item_quantity {
        result = result.replace(i.as_str(), "");
    }
    if !has_quantity {
        result
    } else {
        let mut words = result
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut singularized = false;
        if let Some(word) = words.first_mut() {
            singularize_word(&mut singularized, word);
        }
        if singularized {
            return words.join(" ");
        }

        if let Some(word) = words.last_mut() {
            singularize_word(&mut singularized, word);
        }

        if singularized {
            return words.join(" ");
        }

        if words.len() > 2 {
            for i in 1..words.len() - 2 {
                if let Some(word) = words.get_mut(i) {
                    singularize_word(&mut singularized, word);
                }
            }
        }

        words.join(" ")
    }
}

fn singularize_word(singularized: &mut bool, word: &mut String) {
    let word_singularized = singularize(word);
    if word_singularized != *word {
        *word = word_singularized.to_string();
        *singularized = true;
    }
}
