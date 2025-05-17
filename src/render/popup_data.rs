use crate::addon::Addon;
use crate::api::gw2_wiki::href_to_wiki_url;
use crate::cache::Cache;
use crate::cache::Cacheable;
use crate::cache::CachingStatus;
use crate::context::ui::popup::dimensions::Dimensions;
use crate::context::ui::popup::Popup;
use crate::context::ui::popup::style::Style;
use crate::context::ui::popup::tag_params::TagParams;
use crate::context::ui::popup::token::Token;
use crate::context::Context;
use crate::render::util::ui::{UiAction, COPPER_COLOR, GOLD_COLOR, HIGHLIGHT_COLOR, SILVER_COLOR};
use log::info;
use nexus::imgui::MenuItem;
use nexus::imgui::TreeNodeFlags;
use nexus::imgui::{sys, ChildWindow, MouseButton, Ui};
use std::ptr;
use util::ui::UiLink;
use crate::context::Font;
use crate::render::popup_data::Style::Bold;

use super::util;
use std::thread;
pub const GOLD_COIN_HREF: &str = "/images/thumb/d/d1/Gold_coin.png/18px-Gold_coin.png";
pub const SILVER_COIN_HREF: &str = "/images/thumb/3/3c/Silver_coin.png/18px-Silver_coin.png";
pub const COPPER_COIN_HREF: &str = "/images/thumb/e/eb/Copper_coin.png/18px-Copper_coin.png";

const TEXT_WRAP_LIMIT: usize = 25;

impl Context {
    pub fn render_popup_data(
        ui: &Ui,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if !popup.pinned {
            ui.text(&popup.data.title);
            ui.spacing();
        }
        if Addon::read_config().show_tag_bar {
            Self::render_tag_bar(ui, popup, ui_actions, width_limit);
        }
        if popup.data.is_not_empty() {
            if let Some(_token) = ui.tab_bar(format!("tabs##rps{}", popup.id)) {
                Self::render_general_tab(ui, pinned_popup_vec_index, popup, ui_actions, width_limit, cache, bold_font);
                Self::render_acquisition_tab(ui, pinned_popup_vec_index, popup, ui_actions, width_limit, cache, bold_font);
                Self::render_getting_there_tab(ui, pinned_popup_vec_index, popup, ui_actions, width_limit, cache, bold_font);
                Self::render_contents_tab(ui, pinned_popup_vec_index, popup, ui_actions, width_limit, cache, bold_font);
                Self::render_notes_tab(ui, pinned_popup_vec_index, popup, ui_actions, width_limit, cache, bold_font);
                Self::render_images_tab(ui, pinned_popup_vec_index, popup, ui_actions, cache);
            }
        }
        Self::render_button_ribbon(ui, pinned_popup_vec_index, popup, ui_actions);
    }

    fn render_general_tab(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if Addon::read_config().show_general_tab
            && (!popup.data.description.is_empty() || popup.data.item_ids.is_some())
        {
            let token = ui.tab_item(format!("General##idp{}", popup.id));
            if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
                Self::pin_popup(ui, popup, ui_actions);
            }
            if token.is_some() {
                if !popup.data.description.is_empty() {
                    Self::render_tokens(
                        ui,
                        popup.pinned,
                        &popup.data.description,
                        ui_actions,
                        width_limit,
                        cache,
                        bold_font
                    );
                    ui.new_line();
                }
                Self::render_prices(ui, popup, cache);
            }
        }
    }

    fn render_getting_there_tab(

        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if Addon::read_config().show_getting_there_tab && !popup.data.getting_there.is_empty() {
            let token = ui.tab_item(format!("Getting there##idp{}", popup.id));
            if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
                Self::pin_popup(ui, popup, ui_actions);
            }
            if token.is_some() {
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        popup.pinned,
                        &popup.data.getting_there,
                        ui_actions,
                        width_limit,
                        cache,
                        bold_font
                    );
                };
                if popup.data.getting_there.len() > TEXT_WRAP_LIMIT {
                    let screen_height = ui.io().display_size[1];
                    Self::next_window_size_constraints(
                        [700.0, screen_height * 0.15],
                        [f32::MAX, screen_height * 0.15],
                    );
                    ChildWindow::new(format!("getting_there_scroll##idp{}", popup.id).as_str())
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, render_func);
                    return;
                }
                render_func();
                ui.new_line();
            }
        }
    }
    
    fn render_contents_tab(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if Addon::read_config().show_contents_tab && !popup.data.contents.is_empty() {
            let token = ui.tab_item(format!("Contents##idp{}", popup.id));
            if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
                Self::pin_popup(ui, popup, ui_actions);
            }
            if token.is_some() {
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        popup.pinned,
                        &popup.data.contents,
                        ui_actions,
                        width_limit,
                        cache,
                        bold_font
                    );
                };
                if popup.data.contents.len() > TEXT_WRAP_LIMIT {
                    let screen_height = ui.io().display_size[1];
                    Self::next_window_size_constraints(
                        [700.0, screen_height * 0.15],
                        [f32::MAX, screen_height * 0.15],
                    );
                    ChildWindow::new(format!("contents_scroll##idp{}", popup.id).as_str())
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, render_func);
                    return;
                }
                render_func();
                ui.new_line();
            }
        }
    }

    fn render_acquisition_tab(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if Addon::read_config().show_acquisition_tab && !popup.data.acquisition.is_empty() {
            let token = ui.tab_item(format!("Acquisition##idp{}", popup.id));
            if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
                Self::pin_popup(ui, popup, ui_actions);
            }
            if token.is_some() {
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        popup.pinned,
                        &popup.data.acquisition,
                        ui_actions,
                        width_limit,
                        cache,
                        bold_font
                    );
                };
                if popup.data.acquisition.len() > TEXT_WRAP_LIMIT {
                    let screen_height = ui.io().display_size[1];
                    Self::next_window_size_constraints(
                        [700.0, screen_height * 0.15],
                        [f32::MAX, screen_height * 0.15],
                    );
                    ChildWindow::new(format!("acquisition_scroll##idp{}", popup.id).as_str())
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, render_func);
                    return;
                }
                render_func();
                ui.new_line();
            }
        }
    }

    fn render_notes_tab(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        if Addon::read_config().show_notes_tab && !popup.data.notes.is_empty() {
            let token = ui.tab_item(format!("Notes##idp{}", popup.id));
            if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
                Self::pin_popup(ui, popup, ui_actions);
            }
            if token.is_some() {
                let screen_height = ui.io().display_size[1];
                let mut render_func = || {
                    Self::render_tokens(
                        ui,
                        popup.pinned,
                        &popup.data.notes,
                        ui_actions,
                        width_limit,
                        cache,
                        bold_font
                    );
                };
                if popup.data.notes.len() > TEXT_WRAP_LIMIT {
                    Self::next_window_size_constraints(
                        [700.0, screen_height * 0.15],
                        [f32::MAX, screen_height * 0.15],
                    );
                    ChildWindow::new(format!("notes_scroll##idp{}", popup.id).as_str())
                        .border(true)
                        .scroll_bar(true)
                        .build(ui, render_func);
                    return;
                }
                render_func();
                ui.new_line();
            }
        }
    }

    fn render_images_tab(
        ui: &Ui<'_>, 
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        cache: &mut Cache
    ) {
        if !Addon::read_config().show_images_tab || popup.data.images.is_empty() {
            return;
        }
        let token = ui.tab_item(format!("Images##idp{}", popup.id));
        if ui.is_item_hovered() && pinned_popup_vec_index.is_none() && Addon::read_config().auto_pin_on_tab_hover {
            Self::pin_popup(ui, popup, ui_actions);
        }
        if token.is_some() {
            let mut render_func = || {
                for token in &popup.data.images {
                    match token {
                        Token::Image(href, _) => {
                            let cached_data_opt = cache.textures.retrieve(href.to_string());
                            if let Some(cached_data) = cached_data_opt {
                                if let CachingStatus::Cached = cached_data.caching_status {
                                    if let Some(texture) = cached_data.value() {
                                        let window_width = ui.window_size()[0];
                                        let start_offset =
                                            window_width / 2.0 - texture.width as f32 / 2.0;
                                        ui.set_cursor_pos([start_offset, ui.cursor_pos()[1]]);
                                        ui.invisible_button(
                                            href,
                                            [texture.width as f32, texture.height as f32],
                                        );
                                        ui.get_window_draw_list()
                                            .add_image(
                                                texture.id(),
                                                ui.item_rect_min(),
                                                ui.item_rect_max(),
                                            )
                                            .build();
                                        ui.spacing();
                                    }
                                }
                            }
                        }
                        Token::Text(text, _) => {
                            let words: Vec<&str> = text.split_whitespace().collect();
                            for chunk in words.chunks(4) {
                                let text = chunk.join(" ");
                                let text_size = ui.calc_text_size(&text)[0];
                                let window_width = ui.window_size()[0];
                                ui.set_cursor_pos([
                                    window_width / 2.0 - text_size / 2.0,
                                    ui.cursor_pos()[1],
                                ]);
                                ui.text(text);
                            }
                            ui.spacing();
                        }
                        _ => {}
                    }
                }
                ui.new_line();
            };

            let count = popup
                .data
                .images
                .iter()
                .filter(|item| matches!(item, Token::Image(..)))
                .count();
            if count <= 1 {
                render_func();
                return;
            }

            let screen_height = ui.io().display_size[1];
            Self::next_window_size_constraints(
                [100.0, screen_height * 0.05],
                [400.0, screen_height * 0.30],
            );
            ChildWindow::new(format!("notes_scroll##rps{}", popup.id).as_str())
                .border(true)
                .scroll_bar(true)
                .build(ui, render_func);
        }
    }

    pub fn next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2]) {
        unsafe {
            sys::igSetNextWindowSizeConstraints(
                size_min.into(),
                size_max.into(),
                None,
                ptr::null_mut(),
            )
        }
    }

    fn render_price(ui: &Ui, price: u32, x_pos: Option<f32>, cache: &mut Cache) {
        if let Some(pos) = x_pos {
            ui.set_cursor_screen_pos([pos, ui.cursor_screen_pos()[1]]);
        }
        ui.text_colored(GOLD_COLOR, format!("{:02}", Self::gold_price_part(price)));
        ui.same_line();
        Self::render_image(ui, GOLD_COIN_HREF, &None, cache);
        ui.same_line();
        ui.text_colored(
            SILVER_COLOR,
            format!("{:02}", Self::silver_price_part(price)),
        );
        ui.same_line();
        Self::render_image(ui, SILVER_COIN_HREF, &None, cache);
        ui.same_line();
        ui.text_colored(
            COPPER_COLOR,
            format!("{:02}", Self::copper_price_part(price)),
        );
        ui.same_line();
        Self::render_image(ui, COPPER_COIN_HREF, &None, cache);
    }
    //TODO extract to some kind of converter
    fn gold_price_part(price: u32) -> u32 {
        price / 10000
    }

    fn silver_price_part(price: u32) -> u32 {
        (price % 10000) / 100
    }

    fn copper_price_part(price: u32) -> u32 {
        price % 100
    }

    fn render_tokens(
        ui: &Ui,
        pinned: bool,
        tokens: &Vec<Token>,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        cache: &mut Cache,
        bold_font: &Option<Font>
    ) {
        let style = ui.push_style_var(nexus::imgui::StyleVar::ItemSpacing([0.0, 5.0]));
        ui.spacing();
        let mut starts_with_list = tokens
            .first()
            .is_some_and(|t| matches!(t, Token::ListElement) || matches!(t, Token::Indent(_)));
        let mut current_indent = -1;
        let mut last_token: Option<&Token> = None;
        for token in tokens {
            if !matches!(last_token, Some(Token::Spacing)) {
                ui.same_line();
            }            
            match token {
                Token::Indent(depth) => current_indent = *depth,
                Token::Spacing =>{
                    ui.spacing();
                },
                Token::Text(text, style) => Self::render_text(ui, text, style, current_indent, width_limit, bold_font),
                Token::Tag(tag_params) => {
                    Self::render_tag(
                        ui,
                        tag_params,
                        pinned,
                        ui_actions,
                        current_indent,
                        width_limit,
                    );
                }
                Token::ListElement => Self::render_list_element(ui, &mut starts_with_list, current_indent),
                Token::Image(href, dimensions) => Self::render_image(ui, href, dimensions, cache)
            }
            last_token = Some(token);
            // ui.same_line();
            // Self::handle_line_wrap(ui, current_indent, width_limit);
        }
        style.pop();
    }

    fn render_words<F>(
        ui: &Ui,
        text: &str,
        current_indent: i32,
        width_limit: f32,
        mut render_word: F,
    ) where
        F: FnMut(&Ui, &str),
    {
        let mut first_word = true;
        for word in text.split(" ") {
            if word.is_empty() {
                continue;
            }
            let final_word = if [".", ",", ":"].iter().any(|s| word.starts_with(s)) {
                if (first_word) {
                    first_word = false;
                } else {
                    ui.same_line();
                }
                word.to_string()
            } else {
                Self::handle_line_wrap(ui, current_indent, width_limit, &mut first_word);
                format!(" {}", word)
            };
            render_word(ui, final_word.as_str());
            ui.same_line();
        }
    }

    fn render_text(ui: &Ui, text: &str, style: &Style, current_indent: i32, width_limit: f32, bold_font: &Option<Font>) {
        Self::render_words(
            ui,
            text,
            current_indent,
            width_limit,
            |ui, word| match style {
                Style::Normal => ui.text(word),
                Style::Bold => {
                    if let Some(bold_font) = bold_font {
                        let token = bold_font.push();
                        ui.text(word);
                        drop(token);
                    } else {
                        ui.text_colored(HIGHLIGHT_COLOR, word);
                    }
                },
                Style::Disabled => ui.text_disabled(word),
            },
        );
    }

    fn render_image(ui: &Ui, href: &str, dimensions: &Option<Dimensions>, cache: &mut Cache) {
        let cached_data_opt = cache.textures.retrieve(href.to_string());
        if let Some(cached_data) = cached_data_opt {
            if let CachingStatus::Cached = cached_data.caching_status {
                if let Some(texture) = cached_data.value() {
                    let (width, height) = match dimensions {
                        Some(d) => d.tuple(),
                        None => (texture.width as f32, texture.height as f32)
                    };
                    ui.invisible_button(href, [width as f32, height as f32]);
                    ui.get_window_draw_list()
                        .add_image(texture.id(), ui.item_rect_min(), ui.item_rect_max())
                        .build();
                }
            }
        }
    }

    fn render_tag(
        ui: &Ui,
        tag_params: &TagParams,
        pinned: bool,
        ui_actions: &mut Vec<UiAction>,
        current_indent: i32,
        width_limit: f32,
    ) {
        let href = tag_params.href.to_string();
        let title = tag_params.title.to_string();
        Self::render_words(
            ui,
            &tag_params.text,
            current_indent,
            width_limit,
            |ui, word| {
                ui.text_colored(Addon::read_config().link_color, word);
                if ui.is_item_hovered() && ui.is_mouse_released(MouseButton::Left) && pinned {
                    ui_actions.push(UiAction::Open(UiLink {
                        title: title.clone(),
                        href: href.clone(),
                    }));
                }
            },
        );
    }

    fn render_prices(ui: &Ui<'_>, popup: &mut Popup, cache: &mut Cache) {
        if let Some(item_ids) = &popup.data.item_ids {
            ui.spacing();
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
                        Self::render_price(ui, price.lowest_sell, None, cache);

                        ui.text("Buy ");
                        ui.same_line();
                        Self::render_price(ui, price.highest_buy, Some(sell_text_pos), cache);
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

    fn render_button_ribbon(
        ui: &Ui<'_>,
        pinned_popup_vec_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
    ) {
        if let Some(index) = pinned_popup_vec_index {
            ui.spacing();

            if ui.button(format!("Open wiki##idp{}", popup.id)) {
                let href = popup
                    .data
                    .redirection_href
                    .as_ref()
                    .unwrap_or(&popup.data.href);
                if let Err(err) = open::that_detached(href_to_wiki_url(href)) {
                    log::error!("Failed to open wiki url: {err}");
                }
            }

            ui.same_line();

            if ui.button(format!("More..##idp{}", popup.id)) {
                ui.open_popup(format!("##Popup_idp{}", popup.id));
            }
            ui.popup(format!("##Popup_idp{}", popup.id), || {
                if MenuItem::new(format!("Refresh##idp{}", popup.id)).build(ui) {
                    popup.pos = Some(ui.window_pos());
                    ui_actions.push(UiAction::Refresh(index));
                }
                if ui.is_item_hovered() {
                    ui.tooltip(|| {
                        let formatted_date = popup.data.cached_date.format("%d-%m-%Y %H:%M");
                        ui.text(format!("Last refreshed on {}", formatted_date));
                    });
                }
                if MenuItem::new(format!("Copy name##idp{}", popup.id)).build(ui) {
                    let name = popup.data.title.clone();
                    Addon::lock_threads().push(thread::spawn(move || {
                        let _ = Addon::write_context().clipboard.set_text(name.as_str());
                    }));
                }
            });
        }
    }

    fn render_tag_bar(
        ui: &Ui<'_>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
    ) {
        let tag_iterator = popup.data.tags.iter_mut().enumerate().peekable();
        if tag_iterator.len() > 0 {
            for (_, tag) in tag_iterator {
                let cursor_pos: [f32; 2] = ui.cursor_pos();
                if *cursor_pos.first().unwrap() > width_limit {
                    ui.new_line();
                }
                ui.text_colored(Addon::read_config().link_color, format!("[{}]", tag.1));
                if ui.is_item_hovered() && ui.is_mouse_released(MouseButton::Left) && popup.pinned {
                    ui_actions.push(UiAction::Open(UiLink {
                        href: tag.0.clone(),
                        title: tag.1.clone(),
                    }));
                }
                ui.same_line();
            }
            ui.new_line();
        }
    }

    fn render_list_element(ui: &Ui, starts_with_list: &mut bool, current_indent: i32) {
        if !*starts_with_list {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
        *starts_with_list = false;
        if (Addon::read_config().use_bullet_list_punctuation) {
            ui.text(current_indent.eq(&0).then_some("• ").unwrap_or("- "));
        } else {
            ui.text("- ");
        }
    }

    fn handle_line_wrap(ui: &Ui, current_indent: i32, width_limit: f32, first_word: &mut bool) {
        let cursor_pos = ui.cursor_pos();
        if cursor_pos[0] > width_limit {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }     
    }

    fn add_indent(ui: &Ui, current_indent: i32) {
        if current_indent >= 0 {
            ui.text("    ".repeat(current_indent as usize));
            ui.same_line();
        }
    }
}
