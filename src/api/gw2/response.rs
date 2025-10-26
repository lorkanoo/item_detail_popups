use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PriceApiResponse {
    pub id: u32,
    pub buys: Price,
    pub sells: Price,
}

#[derive(Deserialize, Debug)]
pub struct Price {
    pub unit_price: u32,
}
