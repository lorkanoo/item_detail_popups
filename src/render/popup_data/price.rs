use crate::configuration::popup::rendering_params::RenderingParams;
use crate::state::cache::cache::Cache;
use crate::state::cache::cache::StoreInCache;
use crate::state::context::Context;
use nexus::imgui::Ui;

pub const GOLD_COIN_HREF: &str = "/images/thumb/d/d1/Gold_coin.png/18px-Gold_coin.png";
pub const SILVER_COIN_HREF: &str = "/images/thumb/3/3c/Silver_coin.png/18px-Silver_coin.png";
pub const COPPER_COIN_HREF: &str = "/images/thumb/e/eb/Copper_coin.png/18px-Copper_coin.png";

impl Context {
    pub fn render_price(
        ui: &Ui,
        price: u32,
        cache: &mut Cache,
        rendering_params: &RenderingParams,
    ) {
        let gold_price = format!("{:02}", Self::gold_price_part(price));
        ui.text_colored(rendering_params.gold_coin_color, gold_price);

        ui.same_line();
        Self::render_image(ui, GOLD_COIN_HREF, &None, cache);
        ui.same_line();
        let silver_price = format!("{:02}", Self::silver_price_part(price));
        ui.text_colored(rendering_params.silver_coin_color, silver_price);

        ui.same_line();
        Self::render_image(ui, SILVER_COIN_HREF, &None, cache);
        ui.same_line();
        let copper_price = format!("{:02}", Self::copper_price_part(price));
        ui.text_colored(rendering_params.copper_coin_color, copper_price);

        ui.same_line();
        Self::render_image(ui, COPPER_COIN_HREF, &None, cache);
    }

    pub fn render_prices(
        ui: &Ui<'_>,
        item_ids: &Option<Vec<u32>>,
        cache: &mut Cache,
        rendering_params: &RenderingParams,
        item_quantity: usize,
    ) {
        let Some(item_ids) = item_ids else { return };

        let Some(prices) = cache.prices.retrieve(item_ids.clone()) else {
            return;
        };

        let mut highest_sell_price = None;
        for (item_id, price_data) in &prices {
            if let Some(price) = price_data.value() {
                match highest_sell_price {
                    None => highest_sell_price = Some((*item_id, price.lowest_sell)),
                    Some((_, current_max)) if price.lowest_sell > current_max => {
                        highest_sell_price = Some((*item_id, price.lowest_sell))
                    }
                    _ => {}
                }
            }
        }

        if let Some(price) = highest_sell_price
            .and_then(|(item_id, _)| prices.get(&item_id))
            .and_then(|cached_price| cached_price.value())
        {
            ui.text_disabled(" | ");
            ui.same_line();
            ui.text("Sell ");
            ui.same_line();
            ui.group(|| {
                Self::render_price(ui, price.lowest_sell, cache, rendering_params);
            });
            if ui.is_item_hovered() && item_quantity > 1 {
                ui.tooltip(|| {
                    ui.text_disabled(format!(" Sell {item_quantity} for: "));
                    ui.same_line();
                    Self::render_price(
                        ui,
                        price.lowest_sell * item_quantity as u32,
                        cache,
                        rendering_params,
                    );
                });
            }
            ui.same_line();
            ui.text_disabled(" | ");
            ui.same_line();
            ui.text("Buy ");
            ui.same_line();
            ui.group(|| {
                Self::render_price(ui, price.highest_buy, cache, rendering_params);
            });
            if ui.is_item_hovered() && item_quantity > 1 {
                ui.tooltip(|| {
                    ui.text_disabled(format!(" Buy {item_quantity} for: "));
                    ui.same_line();
                    Self::render_price(
                        ui,
                        price.highest_buy * item_quantity as u32,
                        cache,
                        rendering_params,
                    );
                });
            }
        }
    }

    fn gold_price_part(price: u32) -> u32 {
        price / 10000
    }

    fn silver_price_part(price: u32) -> u32 {
        (price % 10000) / 100
    }

    fn copper_price_part(price: u32) -> u32 {
        price % 100
    }
}
