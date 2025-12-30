use serde::Deserialize;
use crate::api::gw2::price::sell_price::SellPrice;
use crate::api::gw2::price::buy_price::BuyPrice;

#[derive(Deserialize, Debug)]
pub struct PriceApiResponse {
    pub id: u32,
    pub buys: BuyPrice,
    pub sells: SellPrice,
}