use crate::render::ui::UiExtended;
use crate::state::cache::caching_status::CachingStatus;
use crate::state::cache::{Cache, StoreInCache};
use crate::state::context::Context;
use crate::state::popup::dimensions::Dimensions;
use nexus::imgui::Ui;

impl Context {
    pub fn render_image(
        ui: &Ui,
        href: &str,
        dimensions: &Option<Dimensions>,
        cache: &mut Cache,
    ) -> Option<Dimensions> {
        if let Some(output) = dimensions
            .as_ref()
            .filter(|d| ui.not_in_view(&d.height))
            .map(|d| Self::render_placeholder(ui, d, href))
        {
            return output;
        }

        let cached_data_opt = cache.textures.retrieve(href.to_string());
        if let Some(cached_data) = cached_data_opt {
            match cached_data.caching_status {
                CachingStatus::Cached => {
                    if let Some(texture) = cached_data.value() {
                        let (width, height) = match dimensions {
                            Some(d) => d.tuple(),
                            None => (texture.width as f32, texture.height as f32),
                        };
                        ui.invisible_button(href, [width, height]);
                        ui.get_window_draw_list()
                            .add_image(texture.id(), ui.item_rect_min(), ui.item_rect_max())
                            .build();
                        return Some(Dimensions::new(width, height));
                    }
                }
                _ => {
                    return dimensions
                        .as_ref()
                        .and_then(|d| Self::render_placeholder(ui, d, href))
                }
            }
        }
        None
    }

    fn render_placeholder(ui: &Ui, dimensions: &Dimensions, href: &str) -> Option<Dimensions> {
        let (width, height) = dimensions.tuple();
        ui.invisible_button(href, [width, height]);
        Some(dimensions.clone())
    }
}
