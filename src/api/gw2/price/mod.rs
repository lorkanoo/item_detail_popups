use crate::core::http_client::get_sync;

use log::debug;
use serde::Deserialize;
use crate::api::ApiError;
use crate::api::gw2::GW2_API_URL;

#[derive(Deserialize, Debug)]
pub struct PriceApiResponse {
    pub id: u32,
    pub buys: BuyPrice,
    pub sells: SellPrice,
}

#[derive(Deserialize, Debug)]
pub struct BuyPrice {
    pub unit_price: u32,
}

#[derive(Deserialize, Debug)]
pub struct SellPrice {
    pub unit_price: u32,
}

pub fn get_prices(item_ids: &Vec<u32>) -> Result<Vec<PriceApiResponse>, ApiError> {
    debug!("[get_prices] started for {} items", item_ids.len());

    get_sync(prices_path(&item_ids))
        .map_err(|e| ApiError::Unexpected(format!("Failed to fetch prices: {}", e)))
        .and_then(|response| response.into_json::<Vec<PriceApiResponse>>()
            .map_err(|e| ApiError::Internal(format!("Failed to parse prices json: {}", e)))
        )
}

fn prices_path(ids: &[u32]) -> String {
    format!(
        "{}/commerce/prices?ids={}",
        GW2_API_URL,
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

