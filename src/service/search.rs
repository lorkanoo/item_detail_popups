use crate::api::gw2_wiki::{
    extract_search_results, get_wiki_article, get_wiki_special_search, special_search_href,
};
use crate::configuration::search::normalize::Normalize;
use crate::service::popup::fill_popup_with_wiki_details;
use crate::state::cache::StoreInCache;
use crate::state::context::write_context;
use crate::state::popup::Popup;
use crate::state::search::search_result::SearchResult;

pub fn search_wiki(query: &str) -> SearchResult {
    let query_normalized = query.to_string().normalize();
    let item_name_href = format!("/wiki/{}", query_normalized.replace(" ", "_"));
    let mut popup = Popup::new_with(&item_name_href, query_normalized.to_owned(), &1);

    if let Some(cached_data) = write_context()
        .cache
        .popup_data_map
        .retrieve(&item_name_href)
    {
        popup.data = cached_data;
        return SearchResult::SingleMatch(popup);
    }

    if let Some(document) = get_wiki_article(&item_name_href) {
        fill_popup_with_wiki_details(&mut popup, &document);
        write_context()
            .cache
            .popup_data_map
            .store(&item_name_href, &mut popup.data);
        SearchResult::SingleMatch(popup)
    } else {
        let mut matching_search_entries = vec![];
        if let Some(document) = get_wiki_special_search(&special_search_href(item_name_href, None))
        {
            matching_search_entries = extract_search_results(&document, None);
        }
        SearchResult::MultipleMatches(matching_search_entries)
    }
}
