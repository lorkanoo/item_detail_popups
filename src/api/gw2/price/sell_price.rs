use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SellPrice {
    pub unit_price: u32,
}
