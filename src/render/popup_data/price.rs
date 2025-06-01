use crate::cache::Cache;
use crate::cache::Cacheable;
use crate::config::rendering_params::RenderingParams;
use crate::context::Context;
use nexus::imgui::Ui;

pub const GOLD_COIN_HREF: &str = "/images/thumb/d/d1/Gold_coin.png/18px-Gold_coin.png";
pub const SILVER_COIN_HREF: &str = "/images/thumb/3/3c/Silver_coin.png/18px-Silver_coin.png";
pub const COPPER_COIN_HREF: &str = "/images/thumb/e/eb/Copper_coin.png/18px-Copper_coin.png";

impl Context {
    pub fn render_price(
        ui: &Ui,
        price: u32,
        x_render_start_pos: Option<f32>,
        cache: &mut Cache,
        rendering_params: &RenderingParams,
    ) {
        if let Some(pos) = x_render_start_pos {
            ui.set_cursor_screen_pos([pos, ui.cursor_screen_pos()[1]]);
        }
        ui.text_colored(
            rendering_params.gold_coin_color,
            format!("{:02}", Self::gold_price_part(price)),
        );
        ui.same_line();
        Self::render_image(ui, GOLD_COIN_HREF, &None, cache);
        ui.same_line();
        ui.text_colored(
            rendering_params.silver_coin_color,
            format!("{:02}", Self::silver_price_part(price)),
        );
        ui.same_line();
        Self::render_image(ui, SILVER_COIN_HREF, &None, cache);
        ui.same_line();
        ui.text_colored(
            rendering_params.copper_coin_color,
            format!("{:02}", Self::copper_price_part(price)),
        );
        ui.same_line();
        Self::render_image(ui, COPPER_COIN_HREF, &None, cache);
    }

    pub fn render_prices(
        ui: &Ui<'_>,
        item_ids: &Option<Vec<u32>>,
        cache: &mut Cache,
        rendering_params: &RenderingParams,
    ) {
        if let Some(item_ids) = &item_ids {
            let prices_opt = cache.prices.retrieve(item_ids.clone());

            if prices_opt.is_none() {
                return;
            }

            let prices = prices_opt.unwrap();

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
            if let Some((item_id, _)) = highest_sell_price {
                if let Some(price_data) = prices.get(&item_id) {
                    if let Some(price) = price_data.value() {
                        ui.text("Sell ");
                        ui.same_line();
                        let sell_text_pos = ui.cursor_screen_pos()[0];
                        Self::render_price(ui, price.lowest_sell, None, cache, rendering_params);

                        ui.text("Buy ");
                        ui.same_line();
                        Self::render_price(
                            ui,
                            price.highest_buy,
                            Some(sell_text_pos),
                            cache,
                            rendering_params,
                        );
                        if item_ids.len() > 1 {
                            ui.text_disabled("Showing the highest price for item with this name.");
                        }
                    }
                }
            } else {
                ui.text("Sell ");
                ui.text("Buy ");
                if item_ids.len() > 1 {
                    ui.text_disabled("Showing the price of the highest rarity.");
                }
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
