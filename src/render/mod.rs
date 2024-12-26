use crate::addon::Addon;
use crate::api::gw2_wiki::{href_to_wiki_url, prepare_href_popup};
use crate::cache::Cache;
use crate::context::ui::popup::{BasicData, Popup, Style, Token};
use crate::render::util::ui::extended::UiExtended;
use crate::render::util::ui::{
    process_ui_actions_for_vec, UiAction, COPPER_COLOR, GOLD_COLOR, LINK_COLOR, SILVER_COLOR,
    SUCCESS_COLOR,
};
use nexus::imgui::{Condition, StyleVar, Ui, Window};
use std::thread;
use std::time::Duration;
use util::ui::UiLink;

mod options;
pub mod util;

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        if !self.context.run_background_thread {
            return;
        }
        if let Some(progress) = self.context.ui.loading {
            ui.tooltip(|| {
                ui.text(format!("Loading ({}%)", progress));
            });
        }
        self.render_popups(ui);
    }

    fn render_popups(&mut self, ui: &Ui) {
        self.render_hovered_popup(ui);
        self.render_pinned_popups(ui);
    }

    fn render_hovered_popup(&mut self, ui: &Ui) {
        let mut ui_actions: Vec<UiAction> = vec![];
        if let Some(popup) = self.context.ui.hovered_popup.as_mut() {
            Self::render_popup(
                ui,
                None,
                popup,
                &mut ui_actions,
                self.config.price_expiration_sec,
            );
        }
        for ui_action in &ui_actions {
            match ui_action {
                UiAction::Pin => {
                    let popup = self.context.ui.hovered_popup.take().unwrap();
                    self.context.ui.pinned_popups.push(popup);
                }
                UiAction::Close => {
                    self.context.ui.hovered_popup = None;
                }
                _ => {}
            }
        }
    }

    fn render_pinned_popups(&mut self, ui: &Ui) {
        let mut ui_actions = vec![];
        for (i, popup) in self.context.ui.pinned_popups.iter_mut().enumerate() {
            Self::render_popup(
                ui,
                Some(i),
                popup,
                &mut ui_actions,
                self.config.price_expiration_sec,
            );
        }
        process_ui_actions_for_vec(&mut self.context.ui.pinned_popups, &mut ui_actions);
        for ui_action in &ui_actions {
            if let UiAction::Open(ui_link) = ui_action {
                let moved_href = ui_link.href.clone();
                let moved_title = ui_link.title.clone();
                Addon::threads().push(thread::spawn(move || {
                    Addon::lock().context.ui.loading = Some(1);
                    thread::sleep(Duration::from_millis(150));
                    Addon::lock().context.ui.hovered_popup =
                        Some(prepare_href_popup(&moved_href, moved_title));
                    Addon::lock().context.ui.loading = None;
                }));
            }
        }
    }

    fn render_popup(
        ui: &Ui,
        map_index: Option<usize>,
        popup: &mut Popup,
        ui_actions: &mut Vec<UiAction>,
        price_expiration_sec: i64,
    ) {
        if popup.opened && popup.basic_data.pinned {
            let basic_data = &mut popup.basic_data;
            let size = ui.calc_text_size(&basic_data.title);
            let style =
                ui.push_style_var(StyleVar::WindowMinSize([&size[0] * 1.25, &size[1] * 1.0]));
            Window::new(format!("{}##idp{}", basic_data.title.clone(), popup.id))
                .position(basic_data.pos.unwrap_or([0.0, 0.0]), Condition::Appearing)
                .collapsible(false)
                .always_auto_resize(true)
                .save_settings(false)
                .opened(&mut popup.opened)
                .build(ui, || {
                    Self::render_basic_data(
                        ui,
                        map_index,
                        popup.id,
                        basic_data,
                        ui_actions,
                        *ui.window_pos().first().unwrap() + 640.0,
                        price_expiration_sec,
                    );
                });
            style.pop();
        } else {
            if !popup.opened {
                ui.open_popup("##popup_idp");
                popup.opened = true;
            }
            ui.popup("##popup_idp", || {
                let window_pos = ui.window_pos();
                let window_start = window_pos.first().unwrap();
                let width_limit = window_start + 640.0;

                ui.group(|| {
                    Self::render_basic_data(
                        ui,
                        None,
                        popup.id,
                        &mut popup.basic_data,
                        ui_actions,
                        width_limit,
                        price_expiration_sec,
                    );
                });
                Self::close_popup_on_mouse_away(ui, ui_actions);
                if ui.is_item_clicked() && !popup.basic_data.pinned {
                    Self::pin_popup(ui, &mut popup.basic_data, ui_actions);
                }
            });
        }
        if !popup.opened && map_index.is_some() {
            if let Some(index) = map_index {
                ui_actions.push(UiAction::Delete(index));
            }
        }
    }

    fn pin_popup(ui: &Ui, basic_data: &mut BasicData, ui_actions: &mut Vec<UiAction>) {
        ui.close_current_popup();
        ui_actions.push(UiAction::Pin);
        basic_data.pinned = true;
        basic_data.pos = Some(ui.window_pos());
    }

    fn close_popup_on_mouse_away(ui: &Ui, ui_actions: &mut Vec<UiAction>) {
        let mut hover_bounds_min = ui.window_pos();
        hover_bounds_min[0] -= 25.0;
        hover_bounds_min[1] -= 20.0;
        let mut hover_bounds_max = ui.window_pos();
        let size = ui.window_size();
        hover_bounds_max[0] += size[0] + 15.0;
        hover_bounds_max[1] += size[1] + 15.0;
        if !ui.mouse_in_bounds(hover_bounds_min, hover_bounds_max) {
            ui.close_current_popup();
            ui_actions.push(UiAction::Close);
        }
    }

    fn render_basic_data(
        ui: &Ui,
        map_index: Option<usize>,
        popup_id: u64,
        basic_data: &mut BasicData,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
        price_expiration_sec: i64,
    ) {
        if !basic_data.pinned {
            ui.text(&basic_data.title);
            ui.spacing();
        }
        let tag_iterator = basic_data.tags.iter_mut().enumerate().peekable();
        if tag_iterator.len() > 0 {
            for (_, tag) in tag_iterator {
                ui.text_colored(LINK_COLOR, format!("[{}]", tag.1));
                if ui.is_item_clicked() && map_index.is_some() {
                    ui_actions.push(UiAction::Open(UiLink {
                        href: tag.0.clone(),
                        title: tag.1.clone(),
                    }));
                }
                ui.same_line();
                let cursor_pos = ui.cursor_screen_pos();
                if *cursor_pos.first().unwrap() > width_limit {
                    ui.new_line();
                }
            }
            ui.new_line();
        }
        let render_tab_bar = !basic_data.description.is_empty()
            || !basic_data.notes.is_empty()
            || basic_data.item_ids.is_some();
        if render_tab_bar {
            if let Some(_token) = ui.tab_bar(format!("tabs##rps{}", popup_id)) {
                if !basic_data.description.is_empty() || basic_data.item_ids.is_some() {
                    if let Some(_token) = ui.tab_item(format!("General##rps{}", popup_id)) {
                        if !basic_data.description.is_empty() {
                            Self::render_tokens(
                                ui,
                                map_index,
                                &basic_data.description,
                                ui_actions,
                                width_limit,
                            );
                            ui.new_line();
                        }
                        if let Some(item_ids) = &basic_data.item_ids {
                            ui.spacing();
                            let prices = Cache::prices(item_ids.clone(), price_expiration_sec);
                            let mut highest_sell_price = None;
                            for (item_id, price_data) in &prices {
                                if let Some(price) = price_data.value() {
                                    match highest_sell_price {
                                        None => {
                                            highest_sell_price = Some((*item_id, price.lowest_sell))
                                        }
                                        Some((_, current_max))
                                            if price.lowest_sell > current_max =>
                                        {
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
                                        ui.text_colored(
                                            GOLD_COLOR,
                                            &format!("{:02}g ", price.lowest_sell / 10000),
                                        );
                                        ui.same_line();
                                        ui.text_colored(
                                            SILVER_COLOR,
                                            &format!("{:02}s ", (price.lowest_sell % 10000) / 100),
                                        );
                                        ui.same_line();
                                        ui.text_colored(
                                            COPPER_COLOR,
                                            &format!("{:02}c", price.lowest_sell % 100),
                                        );

                                        ui.text("Buy ");
                                        ui.same_line();
                                        ui.set_cursor_screen_pos([
                                            sell_text_pos,
                                            ui.cursor_screen_pos()[1],
                                        ]);
                                        ui.text_colored(
                                            GOLD_COLOR,
                                            &format!("{:02}g ", price.highest_buy / 10000),
                                        );
                                        ui.same_line();
                                        ui.text_colored(
                                            SILVER_COLOR,
                                            &format!("{:02}s ", (price.highest_buy % 10000) / 100),
                                        );
                                        ui.same_line();
                                        ui.text_colored(
                                            COPPER_COLOR,
                                            &format!("{:02}c", price.highest_buy % 100),
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
                }
                if !basic_data.notes.is_empty() {
                    if let Some(_token) = ui.tab_item(format!("Notes##rps{}", popup_id)) {
                        Self::render_tokens(
                            ui,
                            map_index,
                            &basic_data.notes,
                            ui_actions,
                            width_limit,
                        );
                    }
                }
            }
        }

        if map_index.is_some() {
            ui.spacing();
            if ui.button(format!("Open wiki page##rps{}", popup_id)) {
                if let Err(err) = open::that_detached(href_to_wiki_url(&basic_data.href)) {
                    log::error!("Failed to open wiki url: {err}");
                }
            }
        }
    }

    fn render_tokens(
        ui: &Ui,
        map_index: Option<usize>,
        tokens: &Vec<Token>,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
    ) {
        for token in tokens {
            match token {
                Token::Text(text, style) => {
                    for word in text.split(" ") {
                        if word.is_empty() {
                            continue;
                        }
                        if matches!(word, "." | ",") {
                            ui.same_line();
                        }
                        match style {
                            Style::Normal => ui.text(word),
                            Style::Highlighted => ui.text_colored(SUCCESS_COLOR, word),
                        }

                        ui.same_line();

                        let cursor_pos = ui.cursor_screen_pos();
                        if *cursor_pos.first().unwrap() > width_limit {
                            ui.new_line();
                        }
                    }
                }
                Token::Tag(href, text, title) => {
                    ui.text_colored(LINK_COLOR, text);
                    if ui.is_item_clicked() && map_index.is_some() {
                        ui_actions.push(UiAction::Open(UiLink {
                            title: title.clone(),
                            href: href.to_string(),
                        }));
                    }
                }
                Token::ListElement => {
                    ui.same_line();
                    ui.new_line();
                    ui.text("-");
                }
            }
            ui.same_line();
            let cursor_pos = ui.cursor_screen_pos();
            if cursor_pos[0] > width_limit {
                ui.new_line();
            }
        }
        ui.same_line();
    }
}
