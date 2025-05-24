use crate::addon::Addon;
use crate::api::get_sync;
use crate::cache::texture::identifier_to_filename;
use crate::cache::Cacheable;
use crate::config::textures_dir;
use crate::context::ui::popup::style::Style::{self, Bold, Normal};
use ego_tree::NodeRef;
use scraper::selectable::Selectable;
use crate::context::ui::popup::token::Token;
use log::{debug, error, info, trace};
use scraper::{CaseSensitivity, ElementRef, Html, Node, Selector};
use std::fs::{self, File};
use std::io::copy;
use std::ops::Deref;
use crate::context::ui::popup::Popup;
use crate::context::ui::popup::tag_params::TagParams;
use crate::context::ui::popup::popup_data::PopupData;
use crate::context::ui::popup::dimensions::Dimensions;

const GW2_WIKI_URL: &str = "https://wiki.guildwars2.com";
const NO_INDENTATION: i32 = -1;

pub fn href_to_wiki_url(href: &String) -> String {
    debug!("[href_to_wiki_url] Formatting {href}");
    let mut result = format!("{}{}", GW2_WIKI_URL, href.replace('"', "%22"));
    debug!("[href_to_wiki_url] Result {result}");
    result
}

pub fn prepare_item_popup(item_name: &str) -> Popup {
    debug!(
        "[prepare_item_popup] Preparing popup for item: {}",
        item_name
    );
    let item_name_href = format!("/wiki/{}", item_name.replace(" ", "_"));
    let mut popup = prepare_popup(&item_name_href, item_name.to_owned());
    Addon::write_context().ui.loading_progress = Some(10);
    if let Some(mut cached_data) = Addon::write_context()
        .cache
        .popup_data_map
        .retrieve(&item_name_href)
    {
        cached_data.item_ids = popup.data.item_ids.clone();
        cached_data.title = popup.data.title.clone();
        return Popup::new(cached_data);
    }
    if !fill_wiki_details(&item_name_href, &mut popup) {
        Addon::write_context().ui.loading_progress = Some(50);
        if let Some(mut popup) = fill_using_special_search(item_name.to_string(), &mut popup) {
            Addon::write_context()
                .cache
                .popup_data_map
                .store(&item_name_href, &mut popup.data);
            return popup;
        }
    }
    Addon::write_context()
        .cache
        .popup_data_map
        .store(&item_name_href, &mut popup.data);
    popup
}

#[allow(clippy::result_large_err)]
pub fn download_wiki_image(href: &String) -> Result<(), ureq::Error> {
    let path = href_to_wiki_url(href);
    debug!("[download_wiki_image] Downloading image from: {}", path);
    match get_sync(path) {
        Ok(response) => {
            let mut path = textures_dir();
            let _ = fs::create_dir(&path);
            path.push(identifier_to_filename(href));

            debug!(
                "[download_wiki_image] Saving image to \"{}\"",
                path.display()
            );

            let mut file = File::create(&path)?;
            copy(&mut response.into_reader(), &mut file)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn fill_using_special_search(item_name: String, popup: &mut Popup) -> Option<Popup> {
    let id = popup.data.item_ids.as_ref().map(|ids| ids[0]);
    let special_search_result = special_search(id, &special_search_href(item_name, id));
    if let Some(result) = special_search_result {
        let redirection_href = result.0;
        let mut context = Addon::write_context();
        if let Some(mut cached_data) = context.cache.popup_data_map.retrieve(&redirection_href) {
            context
                .cache
                .popup_data_map
                .store(&redirection_href, &mut cached_data);
            return Some(Popup::new(cached_data));
        }
        context.ui.loading_progress = Some(75);
        popup.data.redirection_href = Some(redirection_href.clone());
        drop(context);
        fill_wiki_details(&redirection_href, popup);
        Addon::write_context()
            .cache
            .popup_data_map
            .store(&redirection_href, &mut popup.data);
    }
    None
}

fn special_search_href(item_name: String, item_id: Option<u32>) -> String {
    if let Some(item_id) = item_id {
        format!(
            "/wiki/Special:RunQuery/Search_by_id?title=Special%3ARunQuery%2FSearch_by_id\
            &pfRunQueryFormName=Search+by+id&Search_by_id=id%3D45105%26context%3DItem\
            &wpRunQuery=&pf_free_text=\
            &Search+by+id%5Bid%5D={}\
            &Search+by+id%5Bcontext%5D=Item&wpRunQuery=&pf_free_text=",
            item_id
        )
    } else {
        format!("/index.php?search={item_name}&title=Special%3ASearch&profile=advanced&fulltext=1&ns0=1")
    }
}

pub fn prepare_href_popup(href: &String, title: String) -> Popup {
    debug!(
        "[prepare_href_popup] Preparing popup for href: {} and title: {}",
        href, title
    );
    Addon::write_context().ui.loading_progress = Some(10);
    let cached_data_opt = Addon::write_context().cache.popup_data_map.retrieve(href);
    if let Some(mut cached_data) = cached_data_opt {
        if let Some(item_names) = Addon::write_context().cache.item_names.retrieve(()) {
            cached_data.item_ids = item_names.get(&title).cloned();
        }
        return Popup::new(cached_data);
    }

    let mut popup = prepare_popup(href, title);
    fill_wiki_details(href, &mut popup);
    Addon::write_context()
        .cache
        .popup_data_map
        .store(href, &mut popup.data);
    popup
}

fn prepare_popup(href: &str, title: String) -> Popup {
    debug!(
        "[prepare_popup] Preparing popup for href: {}, title: {}",
        href, title
    );
    let mut data = PopupData {
        title: title.clone(),
        href: href.to_owned(),
        ..PopupData::default()
    };
    if let Some(item_names) = Addon::read_context().cache.item_names.value() {
        data.item_ids = item_names.get(&title).cloned();
    }
    Popup::new(data)
}

pub fn fill_wiki_details(href: &String, popup: &mut Popup) -> bool {
    debug!("[fill_wiki_details] Fetching details for href: {}", href);
    let path = href_to_wiki_url(href);
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                debug!("[fill_wiki_details] response text: {}", text);
                let document = Html::parse_document(&text);

                let exists_selector = Selector::parse(".noarticletext").unwrap();
                if document.select(&exists_selector).next().is_some() {
                    return false;
                }
                fill_item_icon(&document, popup);
                fill_tags(&document, popup);
                fill_description(&document, popup);
                fill_getting_there(&document, popup);
                fill_acquisition(&document, popup);
                fill_contents(&document, popup);
                fill_notes(&document, popup);
                fill_images(&document, popup);
                true
            }
            Err(e) => {
                error!("[fill_wiki_details] failed to fetch text: {}", e);
                false
            }
        },
        Err(e) => {
            error!("[fill_wiki_details] could not fetch data from wiki: {}", e);
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
                    popup.data.tags.insert(href, title.to_string());
                } else if !a_element.inner_html().is_empty() {
                    popup.data.tags.insert(href, a_element.inner_html());
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
        trace!("[skip_to_element] loop");
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

fn next_sibling_of_names<'a>(
    mut next_elem: Option<NodeRef<'a, Node>>,
    element_names: Vec<&str>,
) -> Option<NodeRef<'a, Node>> {
    while let Some(node) = next_elem {
        trace!("[next_sibling_of_name] loop");
        if let Some(element) = ElementRef::wrap(node) {
            if element_names.contains(&element.value().name()) {
                return Some(node);
            }
        }
        next_elem = node.next_sibling();
        continue;
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
        parse_node(&mut description, *start.deref());
        let mut next = start.next_sibling();
        while let Some(node) = next {
            trace!("[fill_description] loop");
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if tag_name != "dl" && tag_name != "ul" {
                    break;
                }
                parse_node(&mut description, node);
            }
            next = node.next_sibling();
        }
    }
    popup.data.description = description;
}

fn fill_getting_there(doc: &Html, popup: &mut Popup) {
    if let Some(doc_pos) = doc.select(
        &Selector::parse("h2:has(#Getting_there)").unwrap()
    ).next() {
        let mut next = doc_pos.next_sibling();
        while let Some(node) = next {
            trace!("[fill_getting_there] loop");
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if !["dl", "ul", "p", "div", "h3", "table"].contains(&tag_name) {
                    break;
                }
                parse_node(&mut popup.data.getting_there, node);
            }
            next = node.next_sibling();
        }
    }
}

fn fill_contents(doc: &Html, popup: &mut Popup) {
    if let Some(doc_pos) = doc.select(
        &Selector::parse("h2:has(#Contents)").unwrap()
    ).next() {
        let mut next = doc_pos.next_sibling();
        while let Some(node) = next {
            trace!("[fill_contents] loop");
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if !["dl", "ul", "p", "div", "h3", "table"].contains(&tag_name) {
                    break;
                }
                parse_node(&mut popup.data.contents, node);
            }
            next = node.next_sibling();
        }
    }
}


fn fill_acquisition(doc: &Html, popup: &mut Popup) {
    if let Some(doc_pos) = doc.select(
        &Selector::parse("h2:has(#Acquisition)").unwrap()
    ).next() {
        let mut next = doc_pos.next_sibling();
        while let Some(node) = next {
            trace!("[fill_acquisition] loop");
            if let Some(element) = ElementRef::wrap(node) {
                let tag_name = element.value().name();
                if !["dl", "ul", "p", "div", "h3", "table"].contains(&tag_name) {
                    break;
                }
                parse_node(&mut popup.data.acquisition, node);
            }
            next = node.next_sibling();
        }

        // let mut next = start.next_sibling();
        // while let Some(node) = next {
        //     trace!("[fill_acquisition] loop");
        //     if let Some(node) = skip_to_element(Some(node), "h3") {
        //         parse_node(&mut acquisition, node);
        //         next = node.next_sibling();
        //         if let Some(div_node) = skip_to_element(node.next_sibling(), "div") {
        //             parse_node(&mut acquisition, div_node);
        //             next = div_node.next_sibling();
        //             continue;
        //         }
        //     } else if let Some(node) = skip_to_element(Some(node), "ul") {
        //         parse_node(&mut acquisition, node);
        //         next = node.next_sibling();
        //     } else if let Some(node) = skip_to_element(Some(node), "table") {
        //         parse_node(&mut acquisition, node);
        //         next = node.next_sibling();
        //     } else {
        //         break;
        //     }
        // }
    }
}

fn fill_notes(document: &Html, popup: &mut Popup) {
    let notes_start_selector = Selector::parse("h2:has(#Notes) + ul").unwrap();
    let mut notes: Vec<Token> = vec![];
    if let Some(start) = document.select(&notes_start_selector).next() {
        parse_node(&mut notes, *start.deref());
        let next = start.next_sibling();
        if let Some(node) = skip_to_element(next, "blockquote") {
            parse_node(&mut notes, node);
        } else if let Some(node) = skip_to_element(next, "table") {
            parse_node(&mut notes, node);
        }
    }
    popup.data.notes = notes;
}

fn fill_images(document: &Html, popup: &mut Popup) {
    let images_start_selector = Selector::parse(".infobox table img, .gallery img").unwrap();
    let mut images: Vec<Token> = vec![];
    let img_elements = document.select(&images_start_selector);
    for img in img_elements {
        let href = img.attr("src");
        if href.is_none() {
            continue;
        }
        images.push(Token::Image(href.unwrap().to_string(), None));
        if let Some(parent) = img.parent() {
            if let Some(next_sibling) = parent.next_sibling() {
                let element = ElementRef::wrap(next_sibling);
                if let Some(element) = element {
                    if element.value().name() == "p" {
                        let text = element.text().collect::<Vec<_>>().join(" ");
                        let processed = process_text(&text);
                        if !processed.is_empty() && !processed.to_lowercase().contains("click") {
                            images.push(Token::Text(processed, Normal));
                        }
                    }
                }
            }
        }
    }
    popup.data.images = images;
}

fn fill_item_icon(document: &Html, popup: &mut Popup) {
    let item_icon_selector = Selector::parse(".infobox-icon img").unwrap();
    if let Some(img) = document.select(&item_icon_selector).next() {
        let href = img.attr("src");
        if href.is_none() {
            return;
        }
        popup.data.item_icon = Some(Token::Image(href.unwrap().to_string(), Some(Dimensions::medium())));
    }
}

// result: href, title
pub fn special_search(item_id: Option<u32>, href: &String) -> Option<(String, String)> {
    let path = href_to_wiki_url(href);
    debug!("[special_search] url {href}");
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                let document = Html::parse_document(&text);
                if let Some(item_id) = item_id {
                    let selector = format!(r#"td[data-sort-value="{}"]"#, item_id);
                    let item_selector = Selector::parse(selector.as_str()).unwrap();
                    if let Some(tag_element) = document.select(&item_selector).next() {
                        if let Some(parent) = tag_element.parent() {
                            if parent.value().is_element() {
                                let element = ElementRef::wrap(parent).unwrap();
                                let link_selector = Selector::parse("a").unwrap();
                                if let Some(a_element) = element.select(&link_selector).next() {
                                    let mut result: (String, String) =
                                        ("".to_string(), "".to_string());
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
                }
                let selector_alternative = r#".mw-search-result-heading a"#.to_string();
                let item_selector_alternative =
                    Selector::parse(selector_alternative.as_str()).unwrap();
                if let Some(tag_element) = document.select(&item_selector_alternative).next() {
                    let mut result: (String, String) = ("".to_string(), "".to_string());
                    if let Some(href) = tag_element.value().attr("href") {
                        result.0 = href.split("#").next().unwrap_or("").to_string();
                    }
                    if let Some(title) = tag_element.value().attr("title") {
                        result.1 = title.to_string();
                    }
                    return Some(result);
                }

                None
            }
            Err(e) => {
                error!("[special_search] failed to fetch text: {}", e);
                None
            }
        },
        Err(e) => {
            debug!("[special_search] could not fetch data from wiki: {}", e);
            None
        }
    }
}

fn parse_node(
    result: &mut Vec<Token>,
    node: NodeRef<Node>,
) {
    parse_node_with_style(result, node, &mut Normal, &mut NO_INDENTATION);
}


fn parse_node_with_style(
    result: &mut Vec<Token>,
    node: NodeRef<Node>,
    style: &mut Style,
    indent_depth: &mut i32,
) {
    if node.value().is_element() {
        let element = ElementRef::wrap(node).unwrap();
        let mut children_iterator = element.children();
        if matches!(element.value().name(), "script" | "sup" | "style" | "table") {
            if element.value().name() == "table" {
                result.push(Token::Spacing);
                result.push(Token::Text(
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
                let child_el = ElementRef::wrap(child);
                if let Some(child_el) = child_el {
                    if let Some(src) = child_el.value().attr("src") {
                        result.push(Token::Image(src.to_string(), Some(Dimensions::small())));
                    }
                } 
                if let Some(text) = child.value().as_text() {
                    let text = process_text(&text.text);
                    let mut title = text.clone();
                    if let Some(title_attr) = element.value().attr("title") {
                        title = process_text(title_attr);
                    }
                    result.push(Token::Tag(TagParams {
                        href: href.split("#").next().unwrap_or("").to_string(),
                        text,
                        title,
                    }));
                }
            }
        } else {
            match element.value().name() {
                "a" | "b" => *style = Bold,
                "ul" => {
                    *indent_depth += 1;
                    result.push(Token::Indent(*indent_depth));
                },
                "li" => result.push(Token::ListElement),
                "dt" | "h3" | "dl" => result.push(Token::Spacing),
                "img" => {
                    if let Some(src) = element.value().attr("src") {
                        result.push(Token::Image(src.to_string(), Some(Dimensions::small())));
                    }
                }
                _ => {}
            }
        }

        for child in children_iterator {
            parse_node_with_style(result, child, style, indent_depth);
        }

        if element.value().name() == "ul" {
            *indent_depth -= 1;
            result.push(Token::Indent(*indent_depth));
        }
        *style = Normal;
    }
    if let Some(text) = node.value().as_text() {
        let processed = process_text(&text.text);
        if !processed.is_empty() {
            result.push(Token::Text(processed, style.clone()));
        }
    }
}

fn process_text(text: &str) -> String {
    let result = text.trim().replace("—", "-").replace("“", "\"").to_string();
    if result == "\"" {
        return "".to_string()
    }
    result
}
