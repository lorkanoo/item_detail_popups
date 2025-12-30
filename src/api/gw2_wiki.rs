use crate::configuration::textures_dir;
use crate::service::http_client::get_sync;
use crate::state::cache::texture::identifier_to_filename;
use crate::state::cache::StoreInCache;
use crate::state::context::write_context;
use crate::state::popup::Popup;
use log::{debug, error, info, warn};
use scraper::selectable::Selectable;
use scraper::{ElementRef, Html, Selector};
use std::fs::{self, File};
use std::io::copy;
use crate::service::popup;
use crate::service::popup::fill_popup_with_wiki_details;
use crate::state::search::matching_entry::MatchingSearchEntry;

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
    let mut popup = Popup::new_with(&item_name_href, item_name.to_owned(), item_quantity);
    write_context().ui.loading_progress = Some(10);
    if let Some(cached_data) = write_context()
        .cache
        .popup_data_map
        .retrieve(&item_name_href)
    {
        popup.data = cached_data;
        return popup;
    }
    if let Some(document) = get_wiki_article(&item_name_href) {
        popup::fill_popup_with_wiki_details(&mut popup, &document);
    } else {
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
    let Some(document) = get_wiki_special_search(&special_search_href(item_name, id)) else { return None };
    let matching_search_entries = extract_search_results(&document, id);
    let Some(search_entry) = matching_search_entries.first() else { return None };
    let mut context = write_context();
    if let Some(mut cached_data) = context.cache.popup_data_map.retrieve(&search_entry.href) {
        context
            .cache
            .popup_data_map
            .store(&search_entry.href, &mut cached_data);
        return Some(Popup::new(cached_data));
    }
    context.ui.loading_progress = Some(75);
    popup.data.redirection_href = Some(search_entry.href.clone());
    drop(context);
    if let Some(document) = get_wiki_article(&search_entry.href) {
        fill_popup_with_wiki_details(popup, &document);
    }
    write_context()
        .cache
        .popup_data_map
        .store(&search_entry.href, &mut popup.data);
    None
}

pub fn special_search_href(item_name: String, item_id: Option<u32>) -> String {
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
    if let Some(document) = get_wiki_article(href) {
        popup::fill_popup_with_wiki_details(&mut popup, &document)
    }
    write_context()
        .cache
        .popup_data_map
        .store(href, &mut popup.data);
    popup
}

fn prepare_popup(href: &str, title: String) -> Popup {
    Popup::new_with(href, title, &1)
}

pub fn get_wiki_article(href: &String) -> Option<Html> {
    debug!("[get_wiki_article] href: {href}");
    let path = href_to_wiki_url(href);

    let Ok(text) = get_sync(path)
        .map_err(|e| format!("[get_wiki_article] could not get wiki article: {e}"))
        .and_then(|resp| {
            resp.into_string()
                .map_err(|e| format!("[get_wiki_article] failed to fetch text: {e}"))
        })
        .inspect_err(|e| warn!("{e}"))
    else {
        return None;
    };

    debug!("[fill_wiki_details] response text: {}", text);
    let document = Html::parse_document(&text);

    let exists_selector = Selector::parse(".noarticletext").unwrap();
    if document.select(&exists_selector).next().is_some() {
        return None;
    }
    Some(document)
}

pub fn get_wiki_special_search(href: &String) -> Option<Html> {
    let path = href_to_wiki_url(href);
    info!("[special_search] url {href}");
    match get_sync(path) {
        Ok(response) => match response.into_string() {
            Ok(text) => {
                Some(Html::parse_document(&text))
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

pub fn extract_search_results(document: &Html, item_id: Option<u32>) -> Vec<MatchingSearchEntry> {
    let mut matching_search_entries = vec![];
    if let Some(item_id) = item_id {
        let selector = format!(r#"td[data-sort-value="{}"]"#, item_id);
        let item_selector = Selector::parse(selector.as_str()).unwrap();
        let mut item_iterator = document.select(&item_selector);
        while let Some(tag_element) = item_iterator.next() {
            if let Some(element) = tag_element.parent().and_then(ElementRef::wrap) {
                let link_selector = Selector::parse("a").unwrap();
                if let Some(link_element) = element.select(&link_selector).next() {
                    let text = extract_title(link_element);
                    let href =  extract_href(link_element);
                    matching_search_entries.push(MatchingSearchEntry::new(text, href));
                }
            }
        }
    } else {
        let selector = r#".mw-search-result-heading > a"#.to_string();
        let item_selector = Selector::parse(selector.as_str()).unwrap();
        let mut item_iterator = document.select(&item_selector);
        while let Some(link_element) = item_iterator.next() {
            let text = extract_title(link_element);
            let href = extract_href(link_element);
            matching_search_entries.push(MatchingSearchEntry::new(text, href));
        }
    }
    matching_search_entries
}

fn extract_title(link_element: ElementRef) -> String {
    link_element.value().attr("title").map(|v| v.to_string()).unwrap_or("".to_string())
}

fn extract_href(element: ElementRef) -> String {
    element
        .value()
        .attr("href")
        .map(|v| v.split("#").next().unwrap_or("").to_string())
        .unwrap_or("".to_string())
}
