pub mod key_combination;
pub mod ui_state;

use crate::addon::Addon;
use singularize::singularize_item_name;

pub fn is_in_game() -> bool {
    let mut is_gameplay = false;
    if let Some(nexus) = unsafe { Addon::read_context().links.nexus() } {
        if nexus.is_gameplay {
            is_gameplay = true;
        }
    }
    is_gameplay
}

pub fn extract_item_name(chat_message: &str) -> Result<String, &'static str> {
    let start = chat_message.find("[");
    let end = chat_message.find("]");
    match (start, end) {
        (Some(start), Some(end)) => {
            if chat_message.len() < 3 || start > end {
                return Err("Could not extract item name: invalid item tag");
            }
            let mut item_tag = chat_message[start + 1..end].to_string();
            item_tag = item_tag.replace("Recipe: ", "");
            Ok(singularize_item_name(&item_tag))
        }
        _ => Err("Could not extract item name: invalid chat message"),
    }
}

pub fn shorten_path(path_str: String) -> String {
    let parts: Vec<&str> = path_str.split(r#"\"#).collect();
    let last_three: Vec<&str> = parts
        .iter()
        .rev()
        .take(3)
        .copied()
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();
    format!("..\\{}", last_three.join("\\"))
}
