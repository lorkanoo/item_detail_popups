use crate::addon::Addon;
use crate::api::get_sync;
use crate::context::ui::popup::Style::{Highlighted, Normal};
use crate::context::ui::popup::{BasicData, Popup, Style, TagParams, Token};
use chrono::{Duration, Local};
use ego_tree::NodeRef;
use function_name::named;
use log::{debug, warn};
use scraper::{CaseSensitivity, ElementRef, Html, Node, Selector};
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

pub fn prepare_item_popup(item_name: &str) -> Popup {
    let item_name_href = format!("/wiki/{}", item_name.replace(" ", "_"));

    let mut popup = prepare_popup(&item_name_href, item_name.to_owned());
    Addon::lock().context.ui.loading = Some(10);
    if let Some(mut value) = retrieve_from_cache(&item_name_href) {
        value.basic_data.item_ids = popup.basic_data.item_ids.clone();
        value.basic_data.title = popup.basic_data.title.clone();
        debug!("Found item IDs: {:?}", value.basic_data.item_ids);
        return value;
    }
    if !fill_wiki_details(&item_name_href, &mut popup) {
        Addon::lock().context.ui.loading = Some(50);
        if let Some(mut value) = fill_using_special_search(&mut popup) {
            Addon::cache().add_popup(&item_name_href, &mut value, true);
            value.basic_data.item_ids = popup.basic_data.item_ids.clone();
            value.basic_data.title = popup.basic_data.title.clone();
            return value;
        }
    }
    Addon::cache().add_popup(&item_name_href, &mut popup, true);
    popup
}

fn fill_using_special_search(popup: &mut Popup) -> Option<Popup> {
    if let Some(item_ids) = &popup.basic_data.item_ids {
        let id = item_ids[0];
        let special_search_result = special_search(id, &special_search_href(id));
        if let Some(result) = special_search_result {
            let href = result.0;
            if let Some(mut value) = retrieve_from_cache(&href) {
                Addon::cache().add_popup(&href, &mut value, true);
                return Some(value);
            }
            Addon::lock().context.ui.loading = Some(75);
            popup.basic_data.href = href.clone();
            fill_wiki_details(&href, popup);
            Addon::cache().add_popup(&href, popup, true);
        }
    }
    None
}

pub fn prepare_href_popup(href: &String, title: String) -> Popup {
    let mut popup = prepare_popup(href, title);
    Addon::lock().context.ui.loading = Some(10);
    if let Some(mut value) = retrieve_from_cache(href) {
        value.basic_data.item_ids = popup.basic_data.item_ids.clone();
        return value;
    }

    fill_wiki_details(href, &mut popup);
    Addon::cache().add_popup(href, &mut popup, true);
    popup
}

fn prepare_popup(href: &str, title: String) -> Popup {
    let mut basic_data = BasicData {
        title: title.clone(),
        href: href.to_owned(),
        ..Default::default()
    };
    if let Some(item_names) = Addon::cache().item_names.value() {
        basic_data.item_ids = item_names.get(&title).cloned();
    }
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
    let path = href_to_wiki_url(href);
    debug!("path: {}", path);
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                let document = Html::parse_document(&text);

                let exists_selector = Selector::parse(".noarticletext").unwrap();
                if document.select(&exists_selector).next().is_some() {
                    return false;
                }

                fill_tags(&document, popup);
                fill_description(&document, popup);
                fill_acquisition(&document, popup);
                fill_notes(&document, popup);

                true
            }
            Err(_) => {
                warn!("[{}] failed to fetch text", function_name!());
                false
            }
        },
        Err(_) => {
            debug!("[{}] could not fetch data from wiki", function_name!());
            false
        }
    }
}

fn fill_tags(document: &Html, popup: &mut Popup) {
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
}

fn skip_to_element<'a>(
    mut next_elem: Option<NodeRef<'a, Node>>,
    element_name: &str,
) -> Option<NodeRef<'a, Node>> {
    while let Some(node) = next_elem {
        if let Some(text) = node.value().as_text() {
            if process_text(text).is_empty() {
                next_elem = node.next_sibling();
                continue;
            }
        }
        if let Some(element) = ElementRef::wrap(node) {
            if element.value().name() == "p"
                && element
                    .value()
                    .has_class("mw-empty-elt", CaseSensitivity::CaseSensitive)
            {
                next_elem = node.next_sibling();
                continue;
            }
            if element.value().name() == element_name {
                return Some(node);
            }
        }
        break;
    }
    None
}

fn fill_description(document: &Html, popup: &mut Popup) {
    let description_start_selector = Selector::parse(
        "div.mw-parser-output > p:not(:has(.wikipopup, script, small)):not(.mw-empty-elt)",
    )
    .unwrap();
    let mut description: Vec<Token> = vec![];

    if let Some(start) = document.select(&description_start_selector).next() {
        if skip_to_element(start.next_sibling(), "h3").is_some() {
            return;
        }
        parse_node(&mut description, *start.deref(), &mut Normal, &mut -1);
        let mut next = start.next_sibling();
        while let Some(node) = next {
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if tag_name != "dl" && tag_name != "ul" {
                    break;
                }
                parse_node(&mut description, node, &mut Normal, &mut -1);
            }
            next = node.next_sibling();
        }
    }
    popup.basic_data.description = description;
}

fn fill_acquisition(document: &Html, popup: &mut Popup) {
    let acquisition_start_selector = Selector::parse("h2:has(#Acquisition)").unwrap();
    let mut acquisition: Vec<Token> = vec![];
    if let Some(start) = document.select(&acquisition_start_selector).next() {
        let mut next = start.next_sibling();
        while let Some(node) = next {
            if let Some(node) = skip_to_element(Some(node), "h3") {
                parse_node(&mut acquisition, node, &mut Normal, &mut -1);
                next = node.next_sibling();
                if let Some(div_node) = skip_to_element(node.next_sibling(), "div") {
                    parse_node(&mut acquisition, div_node, &mut Normal, &mut -1);
                    next = div_node.next_sibling();
                    continue;
                }
            } else if let Some(node) = skip_to_element(Some(node), "ul") {
                parse_node(&mut acquisition, node, &mut Normal, &mut -1);
                next = node.next_sibling();
            } else if let Some(node) = skip_to_element(Some(node), "table") {
                parse_node(&mut acquisition, node, &mut Normal, &mut -1);
                next = node.next_sibling();
            } else {
                break;
            }
        }
    }
    popup.basic_data.acquisition = acquisition;
}

fn fill_notes(document: &Html, popup: &mut Popup) {
    let notes_start_selector = Selector::parse("h2:has(#Notes) + ul").unwrap();
    let mut notes: Vec<Token> = vec![];
    if let Some(start) = document.select(&notes_start_selector).next() {
        parse_node(&mut notes, *start.deref(), &mut Normal, &mut -1);
        let next = start.next_sibling();
        if let Some(node) = skip_to_element(next, "blockquote") {
            parse_node(&mut notes, node, &mut Normal, &mut -1);
        } else if let Some(node) = skip_to_element(next, "table") {
            parse_node(&mut notes, node, &mut Normal, &mut -1);
        }
    }
    popup.basic_data.notes = notes;
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
                            if let Some(a_element) = element.select(&link_selector).next() {
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
            debug!("[{}] could not fetch data from wiki", function_name!());
            None
        }
    }
}

fn parse_node(
    tokens: &mut Vec<Token>,
    node: NodeRef<Node>,
    style: &mut Style,
    indent_depth: &mut i32,
) {
    if node.value().is_element() {
        let element = ElementRef::wrap(node).unwrap();
        let mut children_iterator = element.children();
        if matches!(element.value().name(), "script" | "sup" | "style" | "table") {
            if element.value().name() == "table" {
                tokens.push(Token::Spacing);
                tokens.push(Token::Text(
                    "(open wiki to see the table)".to_string(),
                    Style::Disabled,
                ));
            }
            return;
        }
        if let Some(class) = element.value().attr("class") {
            if class.contains("mw-editsection")
                || class.contains("external")
                || class.contains("extiw")
            {
                return;
            }
        }
        if let Some(style) = element.value().attr("style") {
            if style.contains("display:none") {
                return;
            }
        }
        if let Some(href) = element.value().attr("href") {
            if let Some(child) = children_iterator.next() {
                if let Some(text) = child.value().as_text() {
                    let text = process_text(&text.text);
                    let mut title = text.clone();
                    if let Some(title_attr) = element.value().attr("title") {
                        title = process_text(title_attr);
                    }
                    tokens.push(Token::Tag(TagParams {
                        href: href.split("#").next().unwrap_or("").to_string(),
                        text,
                        title,
                    }));
                }
            }
        } else if matches!(element.value().name(), "a" | "b") {
            *style = Highlighted;
        } else if element.value().name() == "ul" {
            *indent_depth += 1;
            tokens.push(Token::Indent(*indent_depth));
        } else if element.value().name() == "li" {
            tokens.push(Token::ListElement)
        } else if matches!(element.value().name(), "dt" | "h3") {
            tokens.push(Token::Spacing)
        }

        for child in children_iterator {
            parse_node(tokens, child, style, indent_depth);
        }

        if element.value().name() == "ul" {
            *indent_depth -= 1;
            tokens.push(Token::Indent(*indent_depth));
            // tokens.push(Token::Spacing);
        }
        *style = Normal;
    }
    if let Some(text) = node.value().as_text() {
        let processed = process_text(&text.text);
        if !processed.is_empty() {
            tokens.push(Token::Text(processed, style.clone()));
        }
    }
}

fn process_text(text: &str) -> String {
    let result = text.trim().replace("—", "-").replace("“", "\"").to_string();
    if result == "\"" {
        return String::new();
    }
    result
}
