use crate::configuration::{read_config, write_config};
use crate::state::context::write_context;
use crate::state::popup::Popup;
use crate::threads::lock_threads;
use nexus::alert::send_alert;
use std::thread;
use scraper::{CaseSensitivity, ElementRef, Html, Node, Selector};
use log::{debug, trace};
use ego_tree::NodeRef;
use indexmap::IndexMap;
use scraper::selectable::Selectable;
use std::ops::Deref;
use crate::state::popup::dimensions::Dimensions;
use crate::state::popup::popup_data::SectionName;
use crate::state::popup::style::Style;
use crate::state::popup::style::Style::{Bold, Normal};
use crate::state::popup::table_params::{TableCell, TableParams, TableRow};
use crate::state::popup::tag_params::TagParams;
use crate::state::popup::token::Token;

pub fn copy_popup_title(popup: &Popup) {
    let name = popup.data.title.clone();
    lock_threads().push(thread::spawn(move || {
        let _ = write_context().clipboard.set_text(name.as_str());
        if read_config().notification_params.show_item_name_copy_tip {
            send_alert("Tip: You can also copy by right-clicking the item name.");
            write_config().notification_params.show_item_name_copy_tip = false;
        } else {
            send_alert("Item name copied to clipboard.");
        }
    }));
}

pub fn close_all_popups() {
    lock_threads().push(thread::spawn(move || {
        write_context().ui.close_all_popups();
        if read_config().notification_params.show_close_all_tip {
            send_alert("Tip: You can also close all by right-clicking any popup close button.");
            write_config().notification_params.show_close_all_tip = false;
        }
    }));
}

pub fn fill_popup_with_wiki_details(popup: &mut Popup, document: &Html) {
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
}

fn fill_item_icon(document: &Html, popup: &mut Popup) {
    debug!("[fill_item_icon]");
    let item_icon_selector = Selector::parse(".infobox-icon img").unwrap();
    if let Some(img) = document.select(&item_icon_selector).next() {
        let Some(href) = img.attr("src") else { return };

        popup.data.item_icon = Some(Token::Image(href.to_string(), Some(Dimensions::medium())));
    }
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