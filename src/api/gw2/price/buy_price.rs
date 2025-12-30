use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuyPrice {
    pub unit_price: u32,
}