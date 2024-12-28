use crate::addon::Addon;
use crate::api::gw2_wiki::{href_to_wiki_url, prepare_href_popup};
use crate::cache::Cache;
use crate::context::ui::popup::{BasicData, Popup, Style, TagParams, Token};
use crate::render::util::ui::extended::UiExtended;
use crate::render::util::ui::{
    process_ui_actions_for_vec, UiAction, COPPER_COLOR, GOLD_COLOR, LINK_COLOR, SILVER_COLOR,
    SUCCESS_COLOR,
};
use nexus::imgui::{sys, ChildWindow, Condition, Ui, Window};
use std::time::Duration;
use std::{ptr, thread};
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
            let screen_height = ui.io().display_size[1];
            Window::new(format!("{}##idp{}", basic_data.title.clone(), popup.id))
                .position(basic_data.pos.unwrap_or([0.0, 0.0]), Condition::Appearing)
                .collapsible(false)
                .always_auto_resize(true)
                .save_settings(false)
                .opened(&mut popup.opened)
                .size_constraints(
                    [&size[0] * 1.25, &size[1] * 1.0],
                    [f32::MAX, screen_height * 0.4],
                )
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

    fn render_price(ui: &Ui, price: u32, x_pos: Option<f32>) {
        if let Some(pos) = x_pos {
            ui.set_cursor_screen_pos([pos, ui.cursor_screen_pos()[1]]);
        }
        ui.text_colored(GOLD_COLOR, format!("{:02}g ", price / 10000));
        ui.same_line();
        ui.text_colored(SILVER_COLOR, format!("{:02}s ", (price % 10000) / 100));
        ui.same_line();
        ui.text_colored(COPPER_COLOR, format!("{:02}c", price % 100));
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
            || basic_data.item_ids.is_some()
            || !basic_data.acquisition.is_empty();
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
                                        Self::render_price(ui, price.lowest_sell, None);

                                        ui.text("Buy ");
                                        ui.same_line();
                                        Self::render_price(
                                            ui,
                                            price.highest_buy,
                                            Some(sell_text_pos),
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
                if !basic_data.acquisition.is_empty() {
                    if let Some(_token) = ui.tab_item(format!("Acquisition##rps{}", popup_id)) {
                        let screen_height = ui.io().display_size[1];
                        if basic_data.acquisition.len() > 25 {
                            Self::next_window_size_constraints(
                                [700.0, screen_height * 0.15],
                                [f32::MAX, screen_height * 0.15],
                            );
                            ChildWindow::new(
                                format!("acquisition_scroll##rps{}", popup_id).as_str(),
                            )
                            .border(true)
                            .scroll_bar(true)
                            .build(ui, || {
                                Self::render_tokens(
                                    ui,
                                    map_index,
                                    &basic_data.acquisition,
                                    ui_actions,
                                    width_limit,
                                );
                            });
                        } else {
                            Self::render_tokens(
                                ui,
                                map_index,
                                &basic_data.acquisition,
                                ui_actions,
                                width_limit,
                            );
                            ui.new_line();
                        }
                    }
                }
                if !basic_data.notes.is_empty() {
                    if let Some(_token) = ui.tab_item(format!("Notes##rps{}", popup_id)) {
                        let screen_height = ui.io().display_size[1];
                        if basic_data.notes.len() > 25 {
                            Self::next_window_size_constraints(
                                [700.0, screen_height * 0.15],
                                [f32::MAX, screen_height * 0.15],
                            );
                            ChildWindow::new(format!("notes_scroll##rps{}", popup_id).as_str())
                                .border(true)
                                .scroll_bar(true)
                                .build(ui, || {
                                    Self::render_tokens(
                                        ui,
                                        map_index,
                                        &basic_data.notes,
                                        ui_actions,
                                        width_limit,
                                    );
                                });
                        } else {
                            Self::render_tokens(
                                ui,
                                map_index,
                                &basic_data.notes,
                                ui_actions,
                                width_limit,
                            );
                            ui.new_line();
                        }
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

    fn render_tokens(
        ui: &Ui,
        map_index: Option<usize>,
        tokens: &Vec<Token>,
        ui_actions: &mut Vec<UiAction>,
        width_limit: f32,
    ) {
        ui.spacing();
        let mut starts_with_list = tokens.first().map_or(false, |t| {
            matches!(t, Token::ListElement) || matches!(t, Token::Indent(_))
        });
        let mut current_indent = -1;
        for token in tokens {
            match token {
                Token::Indent(depth) => {
                    current_indent = *depth;
                    continue;
                }
                Token::Spacing => {
                    ui.spacing();
                    continue;
                }
                Token::Text(text, style) => {
                    Self::render_text(ui, text, style, current_indent, width_limit);
                }
                Token::Tag(tag_params) => {
                    Self::render_tag(
                        ui,
                        tag_params,
                        map_index,
                        ui_actions,
                        current_indent,
                        width_limit,
                    );
                }
                Token::ListElement => {
                    Self::render_list_element(ui, &mut starts_with_list, current_indent);
                }
            }
            ui.same_line();
            Self::handle_line_wrap(ui, current_indent, width_limit);
        }
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
        for word in text.split(" ") {
            if word.is_empty() {
                continue;
            }
            if matches!(word, "." | ",") {
                ui.same_line();
            }
            render_word(ui, word);
            ui.same_line();
            Self::handle_line_wrap(ui, current_indent, width_limit);
        }
    }

    fn render_text(ui: &Ui, text: &str, style: &Style, current_indent: i32, width_limit: f32) {
        Self::render_words(
            ui,
            text,
            current_indent,
            width_limit,
            |ui, word| match style {
                Style::Normal => ui.text(word),
                Style::Highlighted => ui.text_colored(SUCCESS_COLOR, word),
                Style::Disabled => ui.text_disabled(word),
            },
        );
    }

    fn render_tag(
        ui: &Ui,
        tag_params: &TagParams,
        map_index: Option<usize>,
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
                ui.text_colored(LINK_COLOR, word);
                if ui.is_item_clicked() && map_index.is_some() {
                    ui_actions.push(UiAction::Open(UiLink {
                        title: title.clone(),
                        href: href.clone(),
                    }));
                }
            },
        );
    }

    fn render_list_element(ui: &Ui, starts_with_list: &mut bool, current_indent: i32) {
        if !*starts_with_list {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        }
        *starts_with_list = false;
        ui.text("-");
    }

    fn handle_line_wrap(ui: &Ui, current_indent: i32, width_limit: f32) {
        let cursor_pos = ui.cursor_screen_pos();
        if cursor_pos[0] > width_limit {
            ui.new_line();
            Self::add_indent(ui, current_indent);
        } else {
            ui.same_line();
        }
    }

    fn add_indent(ui: &Ui, current_indent: i32) {
        if current_indent >= 0 {
            ui.text("    ".repeat(current_indent as usize));
            ui.same_line();
        }
    }
}
