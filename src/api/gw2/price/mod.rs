use crate::service::http_client::get_sync;

use crate::api::gw2::GW2_API_URL;
use crate::api::api_error::ApiError;
use log::debug;
use price_api_response::PriceApiResponse;

pub mod price_api_response;
mod buy_price;
mod sell_price;

pub fn get_prices(item_ids: &Vec<u32>) -> Result<Vec<PriceApiResponse>, ApiError> {
    debug!("[get_prices] started for {} items", item_ids.len());

    get_sync(prices_path(&item_ids))
        .map_err(|e| ApiError::Unexpected(format!("Failed to fetch prices: {}", e)))
        .and_then(|response| {
            response
                .into_json::<Vec<PriceApiResponse>>()
                .map_err(|e| ApiError::Internal(format!("Failed to parse prices json: {}", e)))
        })
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
