use crate::core::http_client::get_sync;

use crate::api::ApiError;
use serde::Deserialize;

const GW2TP_URL: &str = "https://api.gw2tp.com";

#[derive(Deserialize, Debug)]
pub struct ItemsResponse {
    pub(crate) items: Vec<(u32, String)>,
}

fn items_name_path() -> String {
    format!("{}/1/bulk/items-names.json", GW2TP_URL)
}

pub fn get_item_names() -> Result<ItemsResponse, ApiError> {
    get_sync(items_name_path())
        .map_err(|e| ApiError::Unexpected(format!("Could not fetch item names: {e}")))
        .and_then(|response| response.into_json()
            .map_err(|e| ApiError::Internal(format!("Failed to fetch json: {e}"))))
}
