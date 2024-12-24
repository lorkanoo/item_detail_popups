use crate::addon::Addon;
use crate::api::get_sync;
use crate::context::ui::popup::Style::{Highlighted, Normal};
use crate::context::ui::popup::{BasicData, Popup, Style, Token};
use chrono::{Duration, Local};
use ego_tree::NodeRef;
use function_name::named;
use log::{debug, warn};
use scraper::{ElementRef, Html, Node, Selector};
use std::ops::Deref;

const GW2_WIKI_URL: &str = "https://wiki.guildwars2.com";

pub fn href_to_wiki_url(href: &String) -> String {
    format!("{}{}", GW2_WIKI_URL, href)
}

fn special_search_href(item_id: u32) -> String {
    format!(
        "/wiki/Special:RunQuery/Search_by_id?title=Special%3ARunQuery%2FSearch_by_id\
        &pfRunQueryFormName=Search+by+id&Search_by_id=id%3D45105%26context%3DItem\
        &wpRunQuery=&pf_free_text=\
        &Search+by+id%5Bid%5D={}\
        &Search+by+id%5Bcontext%5D=Item&wpRunQuery=&pf_free_text=",
        item_id
    )
}

pub fn prepare_item_popup(item_name: &String) -> Popup {
    let item_name_href = format!("/wiki/{}", item_name.replace(" ", "_"));
    if let Some(value) = retrieve_from_cache(&item_name_href) {
        return value;
    }
    let mut popup = prepare_popup(&item_name_href, item_name.clone());
    Addon::lock().context.ui.loading = Some(10);
    if !fill_wiki_details(&item_name_href, &mut popup) {
        Addon::lock().context.ui.loading = Some(50);
        if let Some(mut value) = fill_using_special_search(item_name, &mut popup) {
            Addon::cache().add_popup(&item_name_href, &mut value, true);
            return value;
        }
    }
    Addon::cache().add_popup(&item_name_href, &mut popup, true);
    popup
}

fn fill_using_special_search(item_name: &String, popup: &mut Popup) -> Option<Popup> {
    let mut item_ids = None;
    if let Some(item_names) = &Addon::cache().item_names {
        item_ids = item_names.value.get(item_name).cloned();
    }
    if let Some(item_ids) = item_ids {
        let id = item_ids[0];
        let special_search_result = special_search(id, &special_search_href(id));
        if let Some(result) = special_search_result {
            let href = result.0;
            let title = result.1;
            if let Some(mut value) = retrieve_from_cache(&href) {
                Addon::cache().add_popup(&href, &mut value, true);
                return Some(value);
            }
            Addon::lock().context.ui.loading = Some(75);
            popup.basic_data.title = title.clone();
            popup.basic_data.href = href.clone();
            fill_wiki_details(&href, popup);
            Addon::cache().add_popup(&href, popup, true);
        }
    }
    None
}

pub fn prepare_href_popup(href: &String, title: String) -> Popup {
    if let Some(value) = retrieve_from_cache(href) {
        return value;
    }

    let mut popup = prepare_popup(href, title);
    Addon::lock().context.ui.loading = Some(10);
    fill_wiki_details(href, &mut popup);
    Addon::cache().add_popup(href, &mut popup, true);
    popup
}

fn prepare_popup(href: &String, title: String) -> Popup {
    let mut basic_data = BasicData::default();
    basic_data.title = title.clone();
    basic_data.href = href.clone();

    Popup::new(basic_data)
}

fn retrieve_from_cache(href: &String) -> Option<Popup> {
    let cached_entry = Addon::cache().popups.swap_remove_entry(href);
    if let Some((_, cached)) = cached_entry {
        let cache_expiration = Addon::lock().config.max_popup_cache_expiration;
        if cached.basic_data.cached_date
            + Duration::minutes(cache_expiration.0 * 60 + cache_expiration.1)
            > Local::now()
        {
            let mut result = cached.assign_id_and_clone();
            Addon::cache().add_popup(href, &mut result, false);
            return Some(result);
        }
    }
    None
}

#[named]
pub fn fill_wiki_details(href: &String, popup: &mut Popup) -> bool {
    debug!("[{}] started", function_name!());
    let path = href_to_wiki_url(href);
    debug!("path: {}", path);
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                let document = Html::parse_document(&text);
                let blockquote_selector = Selector::parse(":not(h2) + blockquote").unwrap();
                if let Some(tag_element) = document.select(&blockquote_selector).next() {
                    let link_selector = Selector::parse("a:not(.external, .extiw)").unwrap();
                    for a_element in tag_element.select(&link_selector) {
                        if let Some(href) = a_element.value().attr("href") {
                            let href = href.split("#").next().unwrap_or("").to_string();
                            if let Some(title) = a_element.value().attr("title") {
                                popup.basic_data.tags.insert(href, title.to_string());
                            } else if !a_element.inner_html().is_empty() {
                                popup.basic_data.tags.insert(href, a_element.inner_html());
                            }
                        }
                    }
                }

                let exists_selector = Selector::parse(".noarticletext").unwrap();
                if document.select(&exists_selector).next().is_some() {
                    return false;
                }

                let description_start_selector = Selector::parse(
                    "div.mw-parser-output > p:not(:has(.wikipopup, script, small))",
                )
                .unwrap();
                let mut description: Vec<Token> = vec![];
                if let Some(start) = document.select(&description_start_selector).next() {
                    parse_node(&mut description, *start.deref(), &mut Normal);
                }
                popup.basic_data.description = description;

                let notes_start_selector = Selector::parse("h2:has(#Notes) + ul").unwrap();
                let mut notes: Vec<Token> = vec![];
                if let Some(start) = document.select(&notes_start_selector).next() {
                    parse_node(&mut notes, *start.deref(), &mut Normal);
                }
                popup.basic_data.notes = notes;
                true
            }
            Err(_) => {
                warn!("[{}] failed to fetch text", function_name!());
                false
            }
        },
        Err(_) => {
            warn!("[{}] could not fetch data from wiki", function_name!());
            false
        }
    }
}

#[named]
// result: href, title
pub fn special_search(item_id: u32, href: &String) -> Option<(String, String)> {
    debug!("[{}] started", function_name!());
    let path = href_to_wiki_url(href);
    debug!("path: {}", path);
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                let document = Html::parse_document(&text);
                let selector = format!(r#"td[data-sort-value="{}"]"#, item_id);
                let item_selector = Selector::parse(selector.as_str()).unwrap();
                if let Some(tag_element) = document.select(&item_selector).next() {
                    if let Some(parent) = tag_element.parent() {
                        if parent.value().is_element() {
                            let element = ElementRef::wrap(parent).unwrap();
                            let link_selector = Selector::parse("a").unwrap();
                            for a_element in element.select(&link_selector) {
                                let mut result: (String, String) = ("".to_string(), "".to_string());
                                if let Some(href) = a_element.value().attr("href") {
                                    result.0 = href.split("#").next().unwrap_or("").to_string();
                                }
                                if let Some(title) = a_element.value().attr("title") {
                                    result.1 = title.to_string();
                                }
                                return Some(result);
                            }
                        }
                    }
                }
                None
            }
            Err(_) => {
                warn!("[{}] failed to fetch text", function_name!());
                None
            }
        },
        Err(_) => {
            warn!("[{}] could not fetch data from wiki", function_name!());
            None
        }
    }
}

fn parse_node(tokens: &mut Vec<Token>, node: NodeRef<Node>, style: &mut Style) {
    if node.value().is_element() {
        let element = ElementRef::wrap(node).unwrap();
        let mut children_iterator = element.children();
        if matches!(element.value().name(), "script" | "sup" | "style") {
            return;
        }
        if let Some(style) = element.value().attr("style") {
            if style.contains("display:none") {
                return;
            }
        }
        if let Some(href) = element.value().attr("href") {
            if let Some(child) = children_iterator.next() {
                if let Some(text) = child.value().as_text() {
                    tokens.push(Token::Tag(
                        href.split("#").next().unwrap_or("").to_string(),
                        text.text.trim().to_string(),
                    ));
                } else if let Some(title) = element.value().attr("title") {
                    tokens.push(Token::Tag(
                        href.split("#").next().unwrap_or("").to_string(),
                        title.trim().to_string(),
                    ));
                }
            }
        } else if matches!(element.value().name(), "a" | "b") {
            *style = Highlighted;
        } else if element.value().name() == "li" {
            tokens.push(Token::ListElement)
        }

        for child in children_iterator {
            parse_node(tokens, child, style);
        }
        *style = Normal;
    }
    if let Some(text) = node.value().as_text() {
        let processed = text.text.trim().replace("—", "-").to_string();
        if !processed.is_empty() {
            tokens.push(Token::Text(processed, style.clone()));
        }
    }
}
