use crate::configuration::config::textures_dir;
use crate::core::http_client::get_sync;
use crate::state::cache::cache::StoreInCache;
use crate::state::cache::texture::identifier_to_filename;
use crate::state::context::{read_context, write_context};
use crate::state::popup::dimensions::Dimensions;
use crate::state::popup::popup_data::{PopupData, SectionName};
use crate::state::popup::popup_state::PopupState;
use crate::state::popup::style::Style::{self, Bold, Normal};
use crate::state::popup::table_params::{TableCell, TableParams, TableRow};
use crate::state::popup::tag_params::TagParams;
use crate::state::popup::token::Token;
use crate::state::popup::Popup;
use ego_tree::NodeRef;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use scraper::selectable::Selectable;
use scraper::{CaseSensitivity, ElementRef, Html, Node, Selector};
use std::fs::{self, File};
use std::io::copy;
use std::ops::Deref;

const GW2_WIKI_URL: &str = "https://wiki.guildwars2.com";

pub fn href_to_wiki_url(href: &String) -> String {
    debug!("[href_to_wiki_url] Formatting {href}");
    let result = format!("{}{}", GW2_WIKI_URL, href.replace('"', "%22"));
    debug!("[href_to_wiki_url] Result {result}");
    result
}

pub fn prepare_item_popup_with_quantity(item_name: &str, item_quantity: &usize) -> Popup {
    debug!(
        "[prepare_item_popup] Preparing popup for item: {}",
        item_name
    );
    let item_name_href = format!("/wiki/{}", item_name.replace(" ", "_"));
    let mut popup =
        prepare_popup_with_item_quantity(&item_name_href, item_name.to_owned(), item_quantity);
    write_context().ui.loading_progress = Some(10);
    if let Some(cached_data) = write_context()
        .cache
        .popup_data_map
        .retrieve(&item_name_href)
    {
        popup.data = cached_data;
        return popup;
    }
    if !fill_wiki_details(&item_name_href, &mut popup) {
        write_context().ui.loading_progress = Some(50);
        if let Some(mut popup) = fill_using_special_search(item_name.to_string(), &mut popup) {
            write_context()
                .cache
                .popup_data_map
                .store(&item_name_href, &mut popup.data);
            return popup;
        }
    }
    write_context()
        .cache
        .popup_data_map
        .store(&item_name_href, &mut popup.data);
    debug!(
        "[prepare_item_popup] Popup prepared for item: {}",
        item_name
    );
    popup
}

pub fn prepare_item_popup(item_name: &str) -> Popup {
    prepare_item_popup_with_quantity(item_name, &1)
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
    debug!("[fill_using_special_search]");
    let id = popup.data.item_ids.as_ref().map(|ids| ids[0]);
    let Some(special_search_result) = special_search(id, &special_search_href(item_name, id))
    else {
        return None;
    };

    let redirection_href = special_search_result.0;
    let mut context = write_context();
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
    write_context()
        .cache
        .popup_data_map
        .store(&redirection_href, &mut popup.data);
    None
}

fn special_search_href(item_name: String, item_id: Option<u32>) -> String {
    if let Some(item_id) = item_id {
        format!(
            "/wiki/Special:RunQuery/Search_by_id?title=Special%3ARunQuery%2FSearch_by_id\
            &pfRunQueryFormName=Search+by+id&Search_by_id=id%3D45105%26context%3DItem\
            &wpRunQuery=&pf_free_text=\
            &Search+by+id%5Bid%5D={item_id}\
            &Search+by+id%5Bcontext%5D=Item&wpRunQuery=&pf_free_text={item_id}"
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
    write_context().ui.loading_progress = Some(10);
    let cached_data_opt = write_context().cache.popup_data_map.retrieve(href);
    if let Some(mut cached_data) = cached_data_opt {
        if let Some(item_names) = write_context().cache.item_names.retrieve(()) {
            cached_data.item_ids = item_names.get(&title).cloned();
        }
        return Popup::new(cached_data);
    }

    let mut popup = prepare_popup(href, title);
    fill_wiki_details(href, &mut popup);
    write_context()
        .cache
        .popup_data_map
        .store(href, &mut popup.data);
    popup
}

fn prepare_popup(href: &str, title: String) -> Popup {
    prepare_popup_with_item_quantity(href, title, &1)
}

fn prepare_popup_with_item_quantity(href: &str, title: String, item_quantity: &usize) -> Popup {
    debug!(
        "[prepare_popup_with_quantity] Preparing popup for href: {}, title: {}",
        href, title
    );
    let mut data = PopupData {
        title: title.clone(),
        href: href.to_owned(),
        ..PopupData::default()
    };
    if let Some(item_names) = read_context().cache.item_names.value() {
        data.item_ids = item_names.get(&title).cloned();
    }

    Popup {
        data,
        state: PopupState::new_with_quantity(*item_quantity),
    }
}

pub fn fill_wiki_details(href: &String, popup: &mut Popup) -> bool {
    debug!("[fill_wiki_details] Fetching details for href: {href}");
    let path = href_to_wiki_url(href);

    let Ok(text) = get_sync(path)
        .map_err(|e| format!("[fill_wiki_details] could not fetch data from wiki: {e}"))
        .and_then(|resp| {
            resp.into_string()
                .map_err(|e| format!("[fill_wiki_details] failed to fetch text: {e}"))
        })
        .inspect_err(|e| warn!("{e}"))
    else {
        return false;
    };

    debug!("[fill_wiki_details] response text: {}", text);
    let document = Html::parse_document(&text);

    let exists_selector = Selector::parse(".noarticletext").unwrap();
    if document.select(&exists_selector).next().is_some() {
        return false;
    }
    fill_item_icon(&document, popup);
    fill_tags(&document, popup);
    fill_description(&document, popup);
    let section_selector = Selector::parse("h2").unwrap();
    let sections = document.select(&section_selector);
    for section in sections {
        fill_data(section, &mut popup.data.sections);
    }
    fill_notes(&document, popup);
    fill_images(&document, popup);
    true
}

fn fill_tags(document: &Html, popup: &mut Popup) {
    debug!("[fill_tags]");
    let blockquote_selector = Selector::parse(":not(h2) + blockquote").unwrap();
    let link_selector = Selector::parse("a:not(.external, .extiw)").unwrap();

    if let Some(blockquote) = document.select(&blockquote_selector).next() {
        blockquote.select(&link_selector).for_each(|link| {
            let Some(href) = link
                .value()
                .attr("href")
                .and_then(|href| href.split("#").next())
                .map(|s| s.to_string())
            else {
                return;
            };

            if let Some(title) = link.value().attr("title") {
                popup.data.tags.insert(href.to_string(), title.to_string());
            } else if !link.inner_html().is_empty() {
                popup.data.tags.insert(href.to_string(), link.inner_html());
            }
        })
    }
}

fn skip_to_element<'a>(
    mut next_elem: Option<NodeRef<'a, Node>>,
    element_name: &str,
) -> Option<NodeRef<'a, Node>> {
    while let Some(node) = next_elem {
        trace!("[skip_to_element] loop");

        if node
            .value()
            .as_text()
            .map(|t| process_text(t).is_empty())
            .unwrap_or(false)
        {
            next_elem = node.next_sibling();
            continue;
        }

        let Some(element) = ElementRef::wrap(node) else {
            break;
        };

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

        break;
    }
    None
}

fn fill_description(document: &Html, popup: &mut Popup) {
    debug!("[fill_description]");
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

fn fill_data(doc_pos: ElementRef, sections: &mut IndexMap<SectionName, Vec<Token>>) {
    debug!("[fill_section] {doc_pos:?}");

    let mut data = vec![];
    let headline_selector = Selector::parse(".mw-headline").unwrap();
    let Some(id) = doc_pos
        .select(&headline_selector)
        .next()
        .and_then(|h| h.attr("id"))
        .map(|id| id.replace("_", " "))
    else {
        return;
    };

    let mut next = doc_pos.next_sibling();
    while let Some(node) = next {
        let Some(element) = ElementRef::wrap(node) else {
            next = node.next_sibling();
            continue;
        };

        let tag_name = element.value().name();
        if !["dl", "ul", "p", "div", "h3", "table"].contains(&tag_name) {
            break;
        }
        parse_node(&mut data, node);
        next = node.next_sibling();
    }
    sections.insert(id.to_string(), data);
}

fn fill_notes(document: &Html, popup: &mut Popup) {
    debug!("[fill_notes]");
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
    popup.data.sections.insert("Notes".to_string(), notes);
}

fn fill_images(document: &Html, popup: &mut Popup) {
    debug!("[fill_images]");
    let images_start_selector = Selector::parse(".infobox table img, .gallery img").unwrap();
    let mut images: Vec<Token> = vec![];
    let img_elements = document.select(&images_start_selector);
    for img in img_elements {
        let Some(href) = img.attr("src") else {
            continue;
        };
        images.push(Token::Image(href.to_string(), None));
        if let Some(parent) = img.parent() {
            if let Some(title) = ElementRef::wrap(parent).and_then(|e| e.value().attr("title")) {
                images.push(Token::Text(title.to_string(), Normal));
                continue;
            }
            if let Some(element) = parent
                .next_sibling()
                .and_then(ElementRef::wrap)
                .filter(|e| e.value().name() == "p")
            {
                let text = element.text().collect::<Vec<_>>().join(" ");
                let processed_text = process_text(&text);
                if !processed_text.is_empty() && !processed_text.to_lowercase().contains("click") {
                    images.push(Token::Text(processed_text, Normal));
                }
            }
        }
    }
    popup.data.images = images;
}

fn fill_item_icon(document: &Html, popup: &mut Popup) {
    debug!("[fill_item_icon]");
    let item_icon_selector = Selector::parse(".infobox-icon img").unwrap();
    if let Some(img) = document.select(&item_icon_selector).next() {
        let Some(href) = img.attr("src") else { return };

        popup.data.item_icon = Some(Token::Image(href.to_string(), Some(Dimensions::medium())));
    }
}

// result: href, title
pub fn special_search(item_id: Option<u32>, href: &String) -> Option<(String, String)> {
    let path = href_to_wiki_url(href);
    info!("[special_search] url {href}");
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                let document = Html::parse_document(&text);
                if let Some(item_id) = item_id {
                    let selector = format!(r#"td[data-sort-value="{}"]"#, item_id);
                    let item_selector = Selector::parse(selector.as_str()).unwrap();
                    if let Some(tag_element) = document.select(&item_selector).next() {
                        return tag_element.parent().and_then(ElementRef::wrap).and_then(
                            |element| {
                                let link_selector = Selector::parse("a").unwrap();
                                if let Some(link_element) = element.select(&link_selector).next() {
                                    let mut result: (String, String) =
                                        ("".to_string(), "".to_string());
                                    if let Some(href) = link_element.value().attr("href") {
                                        result.0 = href.split("#").next().unwrap_or("").to_string();
                                    }
                                    if let Some(title) = link_element.value().attr("title") {
                                        result.1 = title.to_string();
                                    }
                                    Some(result)
                                } else {
                                    None
                                }
                            },
                        );
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

fn parse_node(result: &mut Vec<Token>, node: NodeRef<Node>) {
    parse_node_with_style(result, node, &mut Normal, &mut -1);
}

fn parse_node_with_style(
    result: &mut Vec<Token>,
    node: NodeRef<Node>,
    style: &mut Style,
    indent_depth: &mut i32,
) {
    if let Some(element) = ElementRef::wrap(node) {
        parse_element_node(result, style, indent_depth, &element);
    }

    if let Some(text) = node.value().as_text() {
        let processed = process_text(&text.text);
        if !processed.is_empty() {
            result.push(Token::Text(processed, style.clone()));
        }
    }
}

fn parse_element_node(
    result: &mut Vec<Token>,
    style: &mut Style,
    indent_depth: &mut i32,
    element: &ElementRef,
) {
    let mut children_iterator = element.children();
    if matches!(element.value().name(), "script" | "sup" | "style" | "table") {
        if element.value().name() == "table" {
            parse_table(element, result);
        }
        return;
    }
    if let Some(class) = element.value().attr("class") {
        if class.contains("mw-editsection") || class.contains("external") || class.contains("extiw")
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
            "a" | "b" | "dt" => *style = Bold,
            "ul" => {
                *indent_depth += 1;
                result.push(Token::Indent(*indent_depth));
            }
            "li" => result.push(Token::ListElement),
            "h3" | "dl" => result.push(Token::Spacing),
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

fn parse_table(element: &ElementRef, result: &mut Vec<Token>) {
    let mut table_params = TableParams::new();
    table_params.headers = parse_table_headers(element);
    table_params.rows = parse_table_rows(element);
    if table_params.headers.is_empty() {
        let max_cells = table_params
            .rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        for _ in 0..max_cells {
            table_params.headers.push(String::new());
        }
    }
    result.push(Token::Spacing);
    result.push(Token::Table(table_params));
    result.push(Token::Spacing);
}

fn parse_table_rows(element: &ElementRef) -> Vec<TableRow> {
    let mut table_rows = vec![];
    let row_selector = Selector::parse("tbody > tr").unwrap();
    let rows = element.select(&row_selector);
    for row in rows {
        table_rows.push(parse_table_row(&row));
    }
    table_rows
}

fn parse_table_row(row: &ElementRef) -> TableRow {
    let mut table_row = TableRow::new();
    let cell_selector = Selector::parse("tr > td").unwrap();
    let cells = row.select(&cell_selector);
    for cell in cells {
        table_row.cells.push(parse_table_cell(&cell))
    }
    table_row
}

fn parse_table_cell(cell: &ElementRef) -> TableCell {
    let mut table_cell = TableCell::new();
    parse_node(&mut table_cell.tokens, *cell.deref());
    table_cell
}

fn parse_table_headers(element: &ElementRef) -> Vec<String> {
    let mut table_headers = vec![];

    let header_selector = Selector::parse("tbody > tr:first-child > th").unwrap();
    let headers = element.select(&header_selector);
    for header in headers {
        table_headers.push(header.text().collect::<Vec<_>>().join(" "));
    }
    table_headers
}

pub fn process_text(text: &str) -> String {
    let result = text.trim().replace("—", "-").replace("“", "\"").to_string();
    if result == "\"" {
        return "".to_string();
    }
    result
}
